use std::{collections::HashMap, fs};
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

use crate::{Team, Player};

#[derive(Serialize, Deserialize)]
pub struct GameData {
    pub teams: HashMap<u64, Team>,
    pub players: HashMap<u64, Player>,
}


impl GameData {
    pub fn new() -> GameData {
        GameData {
            teams: HashMap::new(),
            players: HashMap::new(),
        }
    }

    pub fn save_as_file(&self) {
        let encoded = serde_json::to_string(&self).unwrap();
        fs::write("league_data.txt", &encoded).unwrap();
    }
}


pub fn new_from_file() -> GameData {
    let file = fs::read_to_string("league_data.txt").unwrap();
    println!("{}", file);
    let decoded: GameData = serde_json::from_str(&file).unwrap();
    decoded
}