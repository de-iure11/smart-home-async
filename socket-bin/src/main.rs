use clap::Parser;
use smart_socket_lib::{Command, Response};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Mutex,
};

#[derive(Parser, Debug)]
#[command(name = "Socket server")]
#[command(about = "Иммитатор умной розетки", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1:7878")]
    listen: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let server_address = args.listen;

    let listener = TcpListener::bind(server_address.clone())
        .await
        .expect("Невозможно привязать TCP-слушатель");

    let smart_socket = Arc::new(Mutex::new(SmartSocket::default()));

    println!("Иммитатор умной розетки запущен на {}", { server_address });

    while let Ok((mut stream, addr)) = listener.accept().await {
        let peer = addr.to_string();
        println!("Установлено соединение {}", { &peer });

        let smart_socket = smart_socket.clone();
        tokio::spawn(async move {
            let mut in_buffer = [0u8];
            while stream.read_exact(&mut in_buffer).await.is_ok() {
                let response = smart_socket
                    .lock()
                    .await
                    .process_command(in_buffer[0].into());
                let response_buf: [u8; 5] = response.into();
                if stream.write_all(&response_buf).await.is_err() {
                    break;
                };
            }

            println!(
                "Соединение с {} потеряно. Ожидаются новые соединения...",
                { &peer }
            );
        });
    }
}

#[derive(Default)]
struct SmartSocket {
    enabled: bool,
}

impl SmartSocket {
    fn process_command(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::TurnOn => {
                self.enabled = true;
                Response::Ok
            }
            Command::TurnOff => {
                self.enabled = false;
                Response::Ok
            }
            Command::GetStatus => {
                if self.enabled {
                    Response::Enabled
                } else {
                    Response::Disabled
                }
            }
            Command::GetPower => {
                if self.enabled {
                    Response::Power(220.5)
                } else {
                    Response::Power(0.0)
                }
            }
            Command::Unknown => {
                println!("Полученна неизвестная команда");
                Response::Unknown
            }
        }
    }
}
