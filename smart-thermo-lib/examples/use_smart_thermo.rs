use smart_thermo_lib::SmartThermo;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    let receiver_address = "127.0.0.1:4343";
    let thermo = SmartThermo::new(receiver_address).await.unwrap();
    for _ in 0..120 {
        time::sleep(Duration::from_secs(1)).await;
        let temperature = thermo.get_temperature().await;
        println!("Текущая температура {temperature}");
    }
}
