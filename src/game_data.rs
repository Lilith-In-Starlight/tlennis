pub struct Game {
    pub away_team: u64,
    pub home_team: u64,
    pub score_home: i32,
    pub score_away: i32,
    pub weather: String,
    pub ball_in: bool, // false => away; true => home ;; starts as true
    pub ticker: Vec<String>,
    pub started: bool, 
}

impl Default for Game {
    fn default() -> Self {
        Self {
            away_team: 0,
            home_team: 1,
            score_home: 0,
            score_away: 0,
            weather: "Daytime".to_string(),
            ball_in: true,
            ticker: Vec::new(),
            started: false,
        }
    }
}

impl Game {
    pub fn tick(&mut self) {
        if &self.ticker.len() > &0 {
            println!("{}", &self.ticker[0]);
            self.ticker.remove(0);
        } else {
            if !&self.started {
                self.ticker.push("Message 1".to_string());
                self.started = true;
                self.ticker.push("Message about a match starting".to_string());
            }
        }
    }
}