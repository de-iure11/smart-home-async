use std::{collections::HashMap, error::Error};

use crate::device::Devices;

pub struct Room {
    pub name: String,
    pub devices: HashMap<String, Devices>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Room {
            name: name.to_string(),
            devices: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn add_device(&mut self, name: String, device: Devices) -> Result<(), Box<dyn Error>> {
        if self.devices.contains_key(&name) {
            let msg = format!("Устройство с именем {} уже существует", name);
            return Err(msg.into());
        };
        self.devices.insert(name, device);
        Ok(())
    }

    pub fn remove_device(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        if !self.devices.contains_key(name) {
            let msg = format!("Устройство с именем {} не найдено", name);
            return Err(msg.into());
        };
        self.devices.remove(name);
        Ok(())
    }

    pub fn get_device(&self, name: &str) -> Option<&Devices> {
        self.devices.get(name)
    }

    pub fn get_mut_device(&mut self, name: &str) -> Option<&mut Devices> {
        self.devices.get_mut(name)
    }

    pub async fn print_report(&mut self) {
        for (name, device) in &mut self.devices {
            println!("{}: {:?}", name, device.get_report().await);
        }
    }
}
