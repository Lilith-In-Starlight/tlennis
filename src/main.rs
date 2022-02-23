mod league_info;
mod player_data;
mod team_data;

// use console::Term;
pub use league_info::*;
pub use player_data::Player;
pub use team_data::Team;



fn main() {
    let mut game_data = LeagueData::new();
    let iterators = Team::new(&mut game_data, "Iterators", '🎲', "Rain World");
    let anglers = Team::new(&mut game_data, "Anglerfish", '🐟', "Dark Bramble");
    Team::new(&mut game_data, "Hobbits", '🧒', "New Zealand");
    Team::new(&mut game_data, "Pop Cats", '😺', "Nyan City");
    Team::new(&mut game_data, "Goats", '🐐', "Underground");
    Team::new(&mut game_data, "Astrologists", '♑', "Paradox Space");
    game_data.save_as_file();

    

}
