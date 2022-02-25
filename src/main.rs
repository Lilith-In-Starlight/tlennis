mod league_info;
mod player_data;
mod team_data;
mod game_data;

use core::time;
use std::thread::sleep;

// use console::Term;
pub use game_data::{Game, GameStates};
pub use league_info::*;
pub use player_data::Player;
pub use team_data::Team;



fn main() {
    let mut league_data = league_info::new_from_file();
    league_data.save_as_file();

    let mut away_id: u64 = 0;
    let mut home_id: u64 = 0;
    {
        for (id, _) in &league_data.teams {
            if away_id == 0 { 
                away_id = *id; 
                println!("{}, {}", away_id, *id);
            }
            else if home_id == 0 { 
                home_id = *id; 
            }
            else { break; }
        }
    }
    let mut game = Game {
        away_team: away_id,
        home_team: home_id,
        ..Default::default()
    };


    loop {
        match game.tick(&mut league_data) {
            GameStates::Play | GameStates::Serving => sleep(time::Duration::from_millis(1000)),
            GameStates::Ended => (),
        }
    }
}
