use std::{collections::HashMap, hash::Hash};

use rand::Rng;

#[derive(Clone)]
pub struct Player {
    pub team: u64,
    pub id: u64,
    pub name: String,
    pub strength: f32,
    pub speed: f32,
}

impl Player {
    pub fn new(players: &mut HashMap<u64, Player>) -> Player {
        let id: u64 = rand::thread_rng().gen();
        let p = Player {
            id: id,
            team: 0,
            name: "Player Tennison".to_string(),
            strength: 5.6,
            speed: 6.5,
        };
        players.insert(id, p.clone());
        p.clone()
    }
}