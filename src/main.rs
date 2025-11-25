use num_format::{Locale, ToFormattedString};
use chrono::{DateTime, Local};
use colored::*;


struct Footballgame {
    home: String,
    away: String,
    home_goals: u32,
    away_goals: u32,
    home_possession: u8,
    away_possession: u8,
    home_totalshots: u8,
    away_totalshots: u8,
    home_chances: u8,
    away_chances: u8,
    home_position: u8,
    away_position: u8,
    stadium_name: String,
    game_attendance: u32,
    knockout_game: bool,
    timeline: DateTime<Local>,
}

impl Footballgame {
    fn total_goals(&self) -> u32 {
        self.home_goals + self.away_goals
    }

    fn result(&self) -> (Option<&str>, Option<&str>) {
        if self.home_goals > self.away_goals {
            (Some(&self.home), Some(&self.away))
        } else if self.home_goals < self.away_goals {
            (Some(&self.away), Some(&self.home))
        } else {
            (None, None)
        }
    }


    fn summarization(&self) -> String {
        let match_type = if self.knockout_game {"Knockout"} else {"League"};

        let actual_attendance = self.game_attendance.to_formatted_string(&Locale::en);

        let home_xg = self.xg(true);
        let away_xg = self.xg(false);

        let home_xg_s = format!("{:.2}", home_xg);
        let away_xg_s = format!("{:.2}", away_xg);

        let (winner_name, loser_name) = self.result();

        let winner_colored = match winner_name {
            Some(n) => n.green().bold(),
            None => "Draw".yellow().bold(),
        };
        let loser_colored = match loser_name {
            Some(n) => n.red().bold(),
            None => "No loser".yellow(),
        };

        let headline = format!(
            "{} thrash {} in North London derby to go {} points clear at the top",
            self.home,
            self.away,
            6
        );
    

    format!(
        "⚽️ Match Summary
-----------------------------------------------------------------------

    Headline: 
    {}

    🏆 Highlights:
    {} {}-{} {}
    {}
    Attendance: {} in attendance

    📊 Match Overview
    Type: {}
    Winner: {}
    Loser: {}
    Total Goals: {}

    🥅 Expected Goals Stats (xG)
    {} xG -> {}
    {} xG -> {}

     ✅ Team Grade:
    {}: {}
    {}: {}

    ⌖ Current Team Position:
    {}: {}
    {}: {}

    🗓️ Match Date:
    {}
    ",
    headline,
    self.home,
    self.home_goals,
    self.away_goals,
    self.away,
    self.stadium_name,
    actual_attendance,
    match_type,
    winner_colored,
    loser_colored,
    self.total_goals(),
    self.home,
    home_xg_s,
    self.away,
    away_xg_s,
    self.home,
    self.grade(true),
    self.away,
    self.grade(false),
    self.home,
    self.home_position,
    self.away,
    self.away_position,
    self.timeline
    )

  }
}

impl Footballgame  {
    fn grade(&self, is_home: bool) -> char {
        let (possession, goals, totalshots, chances) = if is_home {
            (self.home_possession, self.home_goals, self.home_totalshots, self.home_chances)
        } else {
            (self.away_possession, self.away_goals, self.away_totalshots, self.away_chances)
        };

        let mut score = 0;

        if possession >= 70 { score += 6; }
        else if possession >= 55 { score += 5; }
        else if possession >= 50 { score += 3; }


        if goals >= 3 { score += 4; }
        else if goals >= 2 { score += 2; }
        else if goals >= 1 { score += 1; }


        if totalshots >= 12 { score += 3; }
        else if totalshots >= 6 { score += 2; }
        else if totalshots >= 4 { score += 1; }


        if chances >= 7 { score += 4; }
        else if chances >= 6 { score += 2; }
        else if chances >= 5 { score += 1; }


        match score {
            15..=20 => 'A',
            12..=14 => 'B',
            7..=11 => 'C',
            _       => 'D',


        }

    }
}

impl Footballgame {
    fn xg(&self, is_home: bool) -> f32 {
        let (shots, chances, possession, goals) = if is_home {
            (
                self.home_totalshots as f32,
                self.home_chances as f32,
                self.home_possession as f32,
                self.home_goals as f32,
            )
        } else {
            (
                self.away_totalshots as f32,
                self.away_chances as f32,
                self.away_possession as f32,
                self.away_goals as f32,
            )
        };

        let xg = shots * 0.08
            + chances * 0.12
            + (possession / 200.0)
            + goals* 0.15;

        
        (xg * 100.0).round() / 100.0

    }

}



fn main() {
    let game_one = Footballgame {
        home: "Arsenal".to_string(),
        away: "Tottenham Hotspur".to_string(),
        home_goals: 4,
        away_goals: 1,
        home_possession: 57,
        away_possession: 43,
        home_totalshots: 17,
        away_totalshots: 3,
        home_chances: 8,
        away_chances: 2,
        home_position: 1,
        away_position: 9,
        stadium_name: "Fly Emirates Stadium".to_string(),
        game_attendance: 60345,
        knockout_game: false,
        timeline: Local::now(),
    };

    println!("{}", game_one.summarization());


}