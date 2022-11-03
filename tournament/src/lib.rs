use std::{fmt::{Result, Display}, collections::HashMap, cmp::Ordering};

pub fn tally(match_results: &str) -> String {
    let teams = parse(match_results);
    let mut result = String::new();
    result += "Team                           | MP |  W |  D |  L |  P";
    for team in teams {
        result += "\n";
        result += &format!("{}", team).to_string();
    }
    println!("{}", result);
    result
}

fn parse(match_results: &str) -> Vec<Team> {
    let mut teams = HashMap::new();
    for result in match_results.split('\n') {
        if result.is_empty() {
            continue
        }
        let result: Vec<&str> = result.split(';').collect();
        let team1 = result[0];
        let team2 = result[1];
        let result = result[2];

        if let None = teams.get(team1) {
            teams.insert(team1, Team::new(team1));
        }
        if let None = teams.get(team2) {
            teams.insert(team2, Team::new(team2));
        }

        match result {
            "win" => {
                teams.get_mut(team1).unwrap().add_match(MatchResult::Won);
                teams.get_mut(team2).unwrap().add_match(MatchResult::Loss);
            }, 
            "loss" => {
                teams.get_mut(team1).unwrap().add_match(MatchResult::Loss);
                teams.get_mut(team2).unwrap().add_match(MatchResult::Won);
            }, 
            "draw" => {
                teams.get_mut(team1).unwrap().add_match(MatchResult::Draw);
                teams.get_mut(team2).unwrap().add_match(MatchResult::Draw);
            },
            _ =>  unimplemented!("Unsupported result: {}", result),
        } 
    }
    let mut teams: Vec<Team> = teams.values().cloned().collect();
    teams.sort();
    teams
}

enum MatchResult {
    Won, 
    Loss,
    Draw
}

#[derive(Clone, PartialEq, Eq)]
struct Team<'a> {
    name: &'a str,
    played: u32,
    won: u32,
    draw: u32,
}

impl<'a> Team<'a> {
    fn new(name: &str) -> Team{
        Team {
            name: name,
            played: 0,
            won: 0,
            draw: 0,
        }
    }

    fn add_match(&mut self, result: MatchResult) {
        self.played += 1;
        match result {
            MatchResult::Won => self.won += 1,
            MatchResult::Draw => self.draw += 1,
            MatchResult::Loss => {},
        }
    }

    fn loss(&self) -> u32 {
        return self.played - self.won - self.draw;
    }

    fn points(&self) -> u32 {
        return self.won * 3 + self.draw;
    }
}

impl<'a> Display for Team<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let display_name = format!("{: <31}", self.name);
        let display_played = format!("{: >3}", self.played);
        let display_won = format!("{: >3}", self.won);
        let display_draw = format!("{: >3}", self.draw);
        let display_loss = format!("{: >3}", self.loss());
        let display_points = format!("{: >3}", self.points());
        write!(f, "{}|{} |{} |{} |{} |{}", display_name, display_played, display_won, display_draw, display_loss, display_points)
    }
}

impl<'a> PartialOrd for Team<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::Some(self.cmp(other))  
    }
}

impl<'a> Ord for Team<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.points() > other.points() {
            Ordering::Less
        } else if self.points() < other.points() {
            Ordering::Greater
        } else {
            self.name.cmp(other.name)
        }
    }
}