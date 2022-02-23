use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::{LeagueData, Player};

#[derive(Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: u64,
    pub icon: char,
    pub name: String,
    pub location: String,
    pub players: Vec<u64>,
    #[serde(default)]
    pub wins: i64,
    #[serde(default)]
    pub losses: i64,
    #[serde(default)]
    pub modifiers: Vec<String>,
}

impl Default for Team {
    fn default() -> Self {
        Self {
            id: rand::thread_rng().gen(),
            icon: '0',
            name: "Broken Team".to_string(),
            location: "There".to_string(),
            players: Vec::new(),
            wins: 0,
            losses: 0, 
            modifiers: Vec::new(), 
       }
    }
}

impl Team {
    pub fn new(game_data: &mut LeagueData, name: &str, icon: char, location: &str) -> u64 {
        let id: u64 = rand::thread_rng().gen();
        let t = Team {
            id: id,
            name: name.to_string(),
            icon: icon,
            location: location.to_string(),
            ..Default::default()
        };
        game_data.teams.insert(id, t.clone());
        for _ in 0..12 {
            Player::new(game_data, id);
        }

        id
    }

    pub fn fullname(&self) -> String {
        format!("{} {}", &self.location, &self.name)
    }
}
