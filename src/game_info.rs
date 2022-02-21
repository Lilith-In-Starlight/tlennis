use std::{collections::HashMap};

use rand::{Rng};

#[derive(Clone)]
pub struct Player {
    pub id: u64,
    pub team: u64,
    pub name: String,
    pub strength: f32,
    pub speed: f32,
}

#[derive(Clone)]
pub struct Team {
    pub id: u64,
    pub icon: char,
    pub name: String,
    pub location: String,
    pub players: Vec<u64>,
}


pub fn generate_name(name_list: &Vec<&str>, surname_list: &Vec<&str>, general_list: &Vec<&str>) -> String {
    let index_name = rand::thread_rng().gen_range(0..(name_list.len() + general_list.len()));
    let index_surname = rand::thread_rng().gen_range(0..(surname_list.len() + general_list.len()));
    let mut _name = "".to_string();
    if index_name >= name_list.len() {
        _name += general_list[index_name - name_list.len()];
    } else {
        _name += name_list[index_name];
    }
    _name += " ";
    if index_surname >= surname_list.len() {
        _name += general_list[index_surname - surname_list.len()];
    } else {
        _name += surname_list[index_surname];
    }
    // println!("{}", _name);
    _name
}

impl Player {
    pub fn new(players: &mut HashMap<u64, Player>, teams: &mut HashMap<u64, Team>, team: u64) -> u64 {
        let id: u64 = rand::thread_rng().gen();
        let p = Player {
            id: id,
            team: 0,
            name: "Player Tennison".to_string(),
            strength: 5.6,
            speed: 6.5,
        };
        players.insert(id, p.clone());

        if let Some(value) = teams.get_mut(&team) {
            value.players.push(id);
        }

        id
    }
}

impl Team {
    pub fn new(teams: &mut HashMap<u64, Team>, players: &mut HashMap<u64, Player>, name: String, icon: char, location: String) -> u64 {
        let id: u64 = rand::thread_rng().gen();
        let t = Team {
            id: id,
            name: name,
            icon: icon,
            location: location,
            players: Vec::new(),
        };
        teams.insert(id, t.clone());
        for _ in 0..12 {
            Player::new(players, teams, id);
        }

        id
    }
}
