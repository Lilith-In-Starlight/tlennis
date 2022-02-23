use std::{fs};
use once_cell::sync::Lazy;
use rand::{Rng};
use serde::{Serialize, Deserialize};

use crate::LeagueData;


static NAME_LIST: Lazy<Vec<&str>> = Lazy::new(||{
    let contents: &'static str = Box::leak(fs::read_to_string("names.txt").expect("Something went wrong with names.txt").into_boxed_str());
    let result: Vec<&str> = contents.split("\r\n").collect();
    result
});

static SURNAME_LIST: Lazy<Vec<&str>> = Lazy::new(||{
    let contents: &'static str = Box::leak(fs::read_to_string("surnames.txt").expect("Something went wrong with names.txt").into_boxed_str());
    let result: Vec<&str> = contents.split("\r\n").collect();
    result
});

static GENERAL_NAME_LIST: Lazy<Vec<&str>> = Lazy::new(||{
    let contents: &'static str = Box::leak(fs::read_to_string("general.txt").expect("Something went wrong with names.txt").into_boxed_str());
    let result: Vec<&str> = contents.split("\r\n").collect();
    result
});


#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u64,
    pub team: u64,
    pub name: String,
    pub strength: f32,
    pub speed: f32,
    #[serde(default)]
    pub modifiers: Vec<String>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: 0,
            team: 0,
            name: generate_name(),
            strength: rand::thread_rng().gen_range(0.0..6.0),
            speed: rand::thread_rng().gen_range(0.0..6.0),
            modifiers: Vec::new(),
        }
    }
}

pub fn generate_name() -> String {
    let index_name = rand::thread_rng().gen_range(0..(NAME_LIST.len() + GENERAL_NAME_LIST.len()));
    let index_surname = rand::thread_rng().gen_range(0..(SURNAME_LIST.len() + GENERAL_NAME_LIST.len()));
    let mut name = "".to_string();
    if index_name >= NAME_LIST.len() {
        name += GENERAL_NAME_LIST[index_name - NAME_LIST.len()];
    } else {
        name += NAME_LIST[index_name];
    }
    name += " ";

    if rand::thread_rng().gen_range(0..36) == 0 {
        match rand::thread_rng().gen_range(0..2) {
            0 => name += "Mc",
            1 => name += "O'",
            _ => name = name,
        }
    }

    if index_surname >= SURNAME_LIST.len() {
        name += GENERAL_NAME_LIST[index_surname - SURNAME_LIST.len()];
    } else {
        name += SURNAME_LIST[index_surname];
    }

    if rand::thread_rng().gen_range(0..36) == 0 {
         name += "son"
    }
    name
}

impl Player {
    pub fn new(game_data: &mut LeagueData, team: u64) -> u64 {
        let id: u64 = rand::thread_rng().gen();
        let p = Player {
            id: id,
            team: team,
            ..Default::default()
        };
        game_data.players.insert(id, p.clone());

        if let Some(value) = game_data.teams.get_mut(&team) {
            value.players.push(id);
        }

        id
    }
}