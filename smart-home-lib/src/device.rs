use smart_socket_lib::{Command, Response, SmartSocketClient};
use smart_thermo_lib::SmartThermo;

pub enum Devices {
    Socket(SmartSocketClient),
    Thermo(SmartThermo),
}

impl From<SmartSocketClient> for Devices {
    fn from(socket: SmartSocketClient) -> Self {
        Devices::Socket(socket)
    }
}

impl From<SmartThermo> for Devices {
    fn from(thermo: SmartThermo) -> Self {
        Devices::Thermo(thermo)
    }
}

impl Devices {
    pub async fn get_report(&mut self) -> String {
        match self {
            Devices::Socket(smart_socket) => {
                if let Ok(Response::Enabled) = smart_socket.run_command(Command::GetStatus).await {
                    let res = smart_socket.run_command(Command::GetPower).await.ok();
                    match res {
                        Some(res) => return format!("Текущая мощность: {}", res),
                        None => return "Не удалось получить данные".to_string(),
                    };
                }
                "Устройство отключено".to_string()
            }
            Devices::Thermo(smart_thermo) => {
                let current_temperature = smart_thermo.get_temperature().await;
                format!("Текущая температура: {}", current_temperature)
            }
        }
    }
}
