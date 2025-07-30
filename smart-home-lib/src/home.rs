use std::{collections::HashMap, error::Error};

use crate::room::Room;

pub struct Home {
    pub name: String,
    pub rooms: HashMap<String, Room>,
}

impl Home {
    pub fn new(name: &str) -> Self {
        Home {
            name: name.to_string(),
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) -> Result<(), Box<dyn Error>> {
        if self.rooms.contains_key(room.get_name()) {
            let msg = format!("Комната с именем {} уже существует", room.get_name());
            return Err(msg.into());
        };
        self.rooms.insert(room.get_name().to_owned(), room);
        Ok(())
    }

    pub fn remove_room(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        if !self.rooms.contains_key(name) {
            let msg = format!("Комната с именем {} не найдена", name);
            return Err(msg.into());
        };
        self.rooms.remove(name);
        Ok(())
    }

    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    pub fn get_room_mut(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    pub async fn print_report(&mut self) {
        println!("Дом: {}", self.name);
        for (name, room) in &mut self.rooms {
            println!("Комната: {}", name);
            room.print_report().await;
        }
    }
}
