/* Games are the competitions between players, they are not serializable becaue games aren't saved as part of the league */

use rand::Rng;

use crate::LeagueData;

pub struct Game {
    pub away_team: u64,
    pub home_team: u64,
    pub score_home: i32,
    pub score_away: i32,
    pub weather: String,
    pub ball_in_home: bool, // which team will hit the ball || false => away; true => home ;; starts as true
    pub ticker: Vec<String>,
    pub started: bool, // false if serving is necessary ;; might rename
    pub home_players: Vec<usize>,
    pub away_players: Vec<usize>,
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
            started: false,
            home_players: Vec::from([0, 1]),
            away_players: Vec::from([0, 1]),
        }
    }
}

impl Game {
    pub fn tick(&mut self, league_data: &mut LeagueData) {
        if &self.ticker.len() > &0 { // If there's commentary, print it
            println!("{}", &self.ticker[0]);
            self.ticker.remove(0);
        } else { // If there isn't, process the game
            if !&self.started {
                let served_id = league_data.teams[&self.home_team].players[rand::thread_rng().gen_range(0..2)].to_owned();
                self.ticker.push(format!("Message about {} serving for the {}", league_data.players[&served_id].name, league_data.teams[&self.home_team].fullname()));
                self.ball_in_home = !self.ball_in_home;
                self.started = true;
            } else {
                // Which side of the team that hit, hit the ball
                let left_or_right = rand::thread_rng().gen_range(0..2) as usize;
                // Which team hit the ball (away or home)
                let hitting_team_side:&mut Vec<usize> = if self.ball_in_home { &mut self.home_players } else { &mut self.away_players };
                // Which team hit the ball (id)
                let hitting_team_id:&u64 = if self.ball_in_home {&self.home_team} else {&self.away_team};
                // Which player hit the ball (id)
                let hitting_player_id = { // the ID of the player which hit
                    let hitting_player_in_sides:usize  = hitting_team_side[left_or_right]; // team position of the player that hit
                    let which_player_hit = league_data.teams[hitting_team_id].players[hitting_player_in_sides];
                    which_player_hit
                };
                // Name of the player that hit the ball
                let hitting_player_name = league_data.get_player_with_icon(hitting_player_id);
                self.ticker.push(format!("Message about {} hitting the ball", hitting_player_name));


                let hitting_chance: f32 = rand::thread_rng().gen_range(0.0..1.0);
                if hitting_chance < 0.1 {
                    self.ticker.push(format!("Message about {} scoring for their team", hitting_player_name));


                    hitting_team_side[left_or_right] = (hitting_team_side[left_or_right] + 1) % 12;
                }

                
                self.ball_in_home = !self.ball_in_home;
            }
        }
    }
}