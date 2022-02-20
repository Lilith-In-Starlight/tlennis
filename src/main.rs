mod game_info;

use std::{/*io,*/ collections::HashMap};
// use console::Term;
pub use game_info::Player;

#[derive(Clone)]
struct Team {
    id: u64,
    name: String,
    location: String,
    players: Vec<Player>,
}



fn main() {
    let mut _teams: HashMap<u64, Team> = HashMap::new();
    let mut players :HashMap<u64, Player>= HashMap::new();
    Player::new(&mut players);
    for (id, player) in players {
        println!("Player {} has name {}", id, player.name);
    }
    
}
