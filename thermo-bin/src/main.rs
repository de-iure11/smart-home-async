use std::{error::Error, fs, net::SocketAddr, path::Path, time::Duration};

use serde::Deserialize;
use tokio::{
    net::UdpSocket,
    time::{self, Instant},
};

#[derive(Debug, Deserialize)]
struct Config {
    receiver_addr: String,
    interval_secs: f32,
}

#[tokio::main]
async fn main() {
    let config = read_config(Path::new("./thermo-bin/config.toml"))
        .expect("Ожидается корректный файл с настройками");

    println!("Адрес получателя: {}", &config.receiver_addr);

    let receiver_addr = config
        .receiver_addr
        .parse::<SocketAddr>()
        .expect("Ожидается корректный адрес получателя");

    let bind_addr = "127.0.0.1:4320";
    let socket = UdpSocket::bind(bind_addr)
        .await
        .expect("Невозможно привязать сокет");
    let temperature_generator = TemperatureGenerator::default();

    println!("Иммитатор уммного термометра запущен на {}", { bind_addr });

    println!(
        "Отправка температуры с {} на {}",
        &bind_addr, &config.receiver_addr
    );
    loop {
        let temperature = temperature_generator.generate();
        let bytes = temperature.to_be_bytes();
        let send_result = socket.send_to(&bytes, receiver_addr).await;
        if let Err(err) = send_result {
            println!("Не удалось отправить температуру: {}", err);
        }

        let duration = Duration::from_secs_f32(config.interval_secs);
        time::sleep(duration).await;
    }
}

fn read_config(path: &Path) -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

struct TemperatureGenerator {
    started: Instant,
}

impl Default for TemperatureGenerator {
    fn default() -> Self {
        Self {
            started: Instant::now(),
        }
    }
}

impl TemperatureGenerator {
    pub fn generate(&self) -> f32 {
        let delay = Instant::now() - self.started;
        20.0 + (delay.as_secs_f32() / 2.0).sin()
    }
}
