/* Games are the competitions between players, they are not serializable becaue games aren't saved as part of the league */

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use crate::LeagueData;

pub enum GameStates {
    Serving,
    Play,
    Ended,
}
pub struct Game {
    pub away_team: u64,
    pub home_team: u64,
    pub score_home: i32,
    pub score_away: i32,
    pub weather: String,
    pub ball_in_home: bool, // which team will hit the ball || false => away; true => home ;; starts as true
    pub ticker: Vec<String>,
    pub home_players: Vec<usize>,
    pub away_players: Vec<usize>,
    pub game_state: GameStates,
    pub solo_game: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            away_team: 0,
            home_team: 1,
            score_home: 0,
            score_away: 0,
            weather: "Daytime".to_string(),
            ball_in_home: true,
            ticker: Vec::from(["Game start!".to_string()]),
            home_players: Vec::from([0, 1]),
            away_players: Vec::from([0, 1]),
            game_state: GameStates::Serving,
            solo_game: false,
        }
    }
}

impl Game {
    pub fn tick(&mut self, league_data: &mut LeagueData) -> &GameStates {
        let mut rng = Xoshiro256PlusPlus::from_entropy();
        if &self.ticker.len() > &0 { // If there's commentary, print it
            println!("{}", &self.ticker[0]);
            self.ticker.remove(0);
        }
        match self.game_state {
            GameStates::Serving => {
                let served_id = league_data.teams[&self.home_team].players[rng.gen_range(0..100)%2].to_owned();
                self.ticker.push(format!("Message about {} Serving for the {}", league_data.players[&served_id].name, league_data.teams[&self.home_team].fullname()));
                self.ball_in_home = !self.ball_in_home;
                self.game_state = GameStates::Play;
            },
            GameStates::Play => {
                // Which side of the team that hit, hit the ball
                let left_or_right = rng.gen_range(0..100)%2 as usize;
                // Which team hit the ball (away or home)
                let hitting_team_side:&mut Vec<usize>;
                // Which team didn't hit the ball (away or home)
                let other_team_side:&mut Vec<usize>;
                if self.ball_in_home {
                    hitting_team_side = &mut self.home_players;
                    other_team_side = &mut self.away_players;
                } else { 
                    hitting_team_side = &mut self.away_players;
                    other_team_side = &mut self.home_players;
                }
                // Which team hit the ball (id)
                let hitting_team_id:&u64 = if self.ball_in_home {&self.home_team} else {&self.away_team};
                // The non-hitting team (id)
                let other_team_id:&u64 = if self.ball_in_home {&self.away_team} else {&self.home_team};
                // Which player hit the ball (id)
                let hitting_player_id = { // the ID of the player which hit
                    let hitting_player_in_sides:usize  = hitting_team_side[left_or_right]; // team position of the player that hit
                    let which_player_hit = league_data.teams[hitting_team_id].players[hitting_player_in_sides];
                    which_player_hit
                };
                // Name of the player that hit the ball
                let hitting_player_name = league_data.get_player_with_icon(hitting_player_id);
                self.ticker.push(format!("Message about {} hitting the ball", hitting_player_name));


                let other_one_speed:f32;
                let other_two_speed:f32;
                let hitting_chance: f32 = {
                    let hitting_player_strength = league_data.players[&hitting_player_id].strength;
                    let op = league_data.teams[&other_team_id].players[other_team_side[0]];
                    other_one_speed = league_data.players[&op].speed;
                    let op = league_data.teams[&other_team_id].players[other_team_side[1]];
                    other_two_speed = league_data.players[&op].speed;
                    rng.gen_range(0.0..(other_one_speed+other_two_speed+hitting_player_strength*2.0))
                };
                if hitting_chance > other_one_speed+other_two_speed {
                    self.ticker.push(format!("Message about {} scoring for their team", hitting_player_name));
                    if self.ball_in_home {
                        self.score_home += 1;
                    } else {
                        self.score_away += 1;
                    }

                    if self.score_away < 4 && self.score_home < 4 {
                        if !self.solo_game {
                            hitting_team_side[left_or_right] = (hitting_team_side[left_or_right] + 1) % 12;
                            let new_hitting_player = league_data.teams[hitting_team_id].players[hitting_team_side[left_or_right]];
                            self.ticker.push(format!("Message about {} stepping up to the place ({})",
                                league_data.get_player_with_icon(new_hitting_player),
                                self.game_score()));
                        }
                    } else {
                        self.game_state = GameStates::Ended;
                        self.ticker.push(format!("Message about the game ending. The {} win! {}",
                            league_data.teams[hitting_team_id].fullname(),
                            self.game_score()));
                    }
                }


                self.ball_in_home = !self.ball_in_home;
            },
            GameStates::Ended => (),
        }
        &self.game_state
    }

    
    pub fn game_score(&self) -> String {
        if self.score_home == self.score_away {
            if self.score_home != 40 {
                format!("{} All", tennisfy(self.score_home))
            } else {
                format!("Deuce")
            }
        } else {
            format!("{}/{}", tennisfy(self.score_home), tennisfy(self.score_away))
        }
    }
}

pub fn tennisfy<'a>(points:i32) -> &'a str {
    match points {
        0 => "Love",
        1 => "15",
        2 => "30",
        3 => "40",
        _ => "40",
    }
}

