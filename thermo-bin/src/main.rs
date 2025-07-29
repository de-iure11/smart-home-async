use std::{net::SocketAddr, time::Duration};

use tokio::{
    net::UdpSocket,
    time::{self, Instant},
};

#[tokio::main]
async fn main() {
    let args = std::env::args();
    let mut args = args.skip(1);

    let receiver = args.next().unwrap_or_else(|| "127.0.0.1:4321".into());

    println!("Адрес получателя: {}", receiver);

    let receiver = receiver
        .parse::<SocketAddr>()
        .expect("Ожидается корректный адрес получателя");

    let bind_addr = "127.0.0.1:4320";
    let socket = UdpSocket::bind(bind_addr)
        .await
        .expect("Невозможно привязать сокет");
    let temperature_generator = TemperatureGenerator::default();

    println!("Отправка температуры с {bind_addr} на {receiver}");
    loop {
        let temperature = temperature_generator.generate();
        let bytes = temperature.to_be_bytes();
        let send_result = socket.send_to(&bytes, receiver).await;
        if let Err(err) = send_result {
            println!("Не удалось отправить температуру: {}", err);
        }

        let duration = Duration::from_secs_f32(0.5);
        time::sleep(duration).await;
    }
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
