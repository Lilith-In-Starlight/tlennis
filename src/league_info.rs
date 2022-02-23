use std::{collections::HashMap, fs};
use serde::{Serialize, Deserialize};
use serde_json::*;

use crate::{Team, Player};

#[derive(Serialize, Deserialize)]
pub struct LeagueData {
    pub teams: HashMap<u64, Team>,
    pub players: HashMap<u64, Player>,
}


impl LeagueData {
    pub fn new() -> LeagueData {
     LeagueData {
            teams: HashMap::new(),
            players: HashMap::new(),
        }
    }

    pub fn save_as_file(&self) {
        let encoded = serde_json::to_string(&self).unwrap();
        fs::write("league_data.txt", &encoded).unwrap();
    }

    

    pub fn set_player_team(&mut self, player_id: u64, new_team: u64) {
        let mut p = self.players.get_mut(&player_id).unwrap();
        if let Some(team) = self.teams.get_mut(&p.team) {
            if let Some(position) = team.players.iter().position(|&r| &r == &p.id) {
                team.players.remove(position);
            }
        }

        if let Some(team) = self.teams.get_mut(&new_team) {
            if let None = team.players.iter().position(|&r| &r == &p.id) {
                team.players.push(p.id);
            }
        }
        p.id = new_team;
    }
}


pub fn new_from_file() -> LeagueData {
    let file = fs::read_to_string("league_data.txt").unwrap();
    println!("{}", file);
    let decoded: LeagueData = serde_json::from_str(&file).unwrap();
    decoded
}