use smart_home_lib::{home::Home, room::Room};
use smart_socket_lib::SmartSocketClient;
use smart_thermo_lib::SmartThermo;
use std::process::Command;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut process_thermo = Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("thermo-bin")
        .spawn()
        .expect("Не удалось запустить иммитатор умного термометра");

    let process_thermo_pid = process_thermo.id();
    println!(
        "-- Дочерний процесс (иммитатор умного термометра) запущен с PID: {}",
        process_thermo_pid
    );

    let mut process_socket = Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("socket-bin")
        .arg("--")
        .arg("--listen")
        .arg("127.0.0.1:7899")
        .spawn()
        .expect("Не удалось запустить иммитатор умной розетки");

    let process_socket_pid = process_socket.id();
    println!(
        "-- Дочерний процесс (иммитатор умной розетки) запущен с PID: {}",
        process_socket_pid
    );

    tokio::time::sleep(Duration::from_secs(2)).await;

    test_smart_home().await;

    println!("* * *");

    // Завершаем процессы
    process_thermo
        .kill()
        .expect("Не удалось завершить процесс иммитатора умного термометра");

    process_socket
        .kill()
        .expect("Не удалось убить процесс иммитатора умной розетки");

    tokio::time::sleep(Duration::from_secs(2)).await;

    // Проверяем, завершились ли процессы
    if let Some(exit_status) = process_thermo.try_wait().unwrap() {
        println!(
            "Процесс термометра завершен с кодом: {:?}",
            exit_status.code()
        );
    } else {
        println!("Процесс термометра ещё не завершился.");
    }

    if let Some(exit_status) = process_socket.try_wait().unwrap() {
        println!("Процесс розетки завершен с кодом: {:?}", exit_status.code());
    } else {
        println!("Процесс розетки ещё не завершился.");
    }
}

async fn test_smart_home() {
    println!("Запуск теста SmartHome...");

    let mut home = Home::new("My Home");

    let room_1 = Room::new("Room 1");
    let room_2 = Room::new("Room 2");

    let mut socket_1 = SmartSocketClient::new("127.0.0.1:7899").await.unwrap();
    socket_1
        .run_command(smart_socket_lib::Command::TurnOn)
        .await
        .unwrap();

    let socket_2 = SmartSocketClient::new("127.0.0.1:7899").await.unwrap();

    let thermo_1 = SmartThermo::new("127.0.0.1:4343").await.unwrap();
    let thermo_2 = SmartThermo::new("127.0.0.1:4344").await.unwrap();

    home.add_room(room_1).unwrap();
    home.add_room(room_2).unwrap();

    let room_1 = home.get_room_mut("Room 1").unwrap();

    room_1
        .add_device("socket 1".to_string(), socket_1.into())
        .unwrap();
    room_1
        .add_device("thermo 1".to_string(), thermo_1.into())
        .unwrap();

    let room_2 = home.get_room_mut("Room 2").unwrap();

    room_2
        .add_device("socket 2".to_string(), socket_2.into())
        .unwrap();
    room_2
        .add_device("thermo 2".to_string(), thermo_2.into())
        .unwrap();

    tokio::time::sleep(Duration::from_secs(5)).await;

    let _ = home.print_report().await;
}
