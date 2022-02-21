mod game_info;

use std::{/*io,*/ collections::HashMap, fs};
// use console::Term;
pub use game_info::*;
pub use game_info::Player;
pub use game_info::Team;


fn main() {

    let contents = fs::read_to_string("names.txt").expect("Something went wrong with names.txt");
    let names: Vec<&str> = contents.split("\r\n").collect();

    let contents = fs::read_to_string("surnames.txt").expect("Something went wrong with surnames.txt");
    let last_names: Vec<&str> = contents.split("\r\n").collect();

    let contents = fs::read_to_string("general.txt").expect("Something went wrong with general.txt");
    let general_names: Vec<&str> = contents.split("\r\n").collect();

    drop(&contents);


    let mut teams: HashMap<u64, Team> = HashMap::new();
    let mut players :HashMap<u64, Player>= HashMap::new();
    generate_name(&names, &last_names, &general_names);
    let _t = Team::new(&mut teams, &mut players, "uwu".to_string(), 'a', "a".to_string());
    for _ in 0..12 {
        let name = generate_name(&names, &last_names, &general_names);
        println!("{}", name);
    }
}
