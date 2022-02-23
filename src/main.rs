mod league_info;
mod player_data;
mod team_data;

// use console::Term;
pub use league_info::*;
pub use player_data::Player;
pub use team_data::Team;



fn main() {
    let mut game_data = LeagueData::new();
    game_data.save_as_file();
    
}
