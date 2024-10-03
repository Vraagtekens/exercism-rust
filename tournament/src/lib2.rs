use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct Team {
    name: String,
    wins: usize,
    draw: usize,
    loses: usize,
}

impl Team {
    fn new(name: String) -> Self {
        Self {
            name,
            wins: 0,
            draw: 0,
            loses: 0,
        }
    }

    fn to_string(self) -> String {
        let mp = self.wins + self.draw + self.loses;
        let points = self.wins * 3 + self.draw * 1;

        // print!("{:?}", self);

        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name,  // Left-aligned, width of 31 for team name
            mp,         // Right-aligned, width of 2 for matches played
            self.wins,  // Right-aligned, width of 2 for wins
            self.draw,  // Right-aligned, width of 2 for draws
            self.loses, // Right-aligned, width of 2 for losses
            points      // Right-aligned, width of 2 for points
        )
    }

    fn update_stats(&mut self, outcome: &str, is_team1: bool) {
        match (outcome, is_team1) {
            ("win", true) | ("loss", false) => self.wins += 1,
            ("loss", true) | ("win", false) => self.loses += 1,
            ("draw", _) => self.draw += 1,
            _ => {}
        }
    }
}

// Implement PartialEq and Eq to compare teams by name only.
impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Team {}

// Implement Hash so that only the name is considered for hashing.
impl Hash for Team {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub fn tally(match_results: &str) -> String {
    // todo!("Given the result of the played matches '{match_results}' return a properly formatted tally table string.");

    if match_results.is_empty() {
        return "Team                           | MP |  W |  D |  L |  P".to_string();
    }

    let mut teams: HashMap<String, Team> = HashMap::new();
    let match_results = match_results.split("\n");

    for result in match_results {
        let split: Vec<&str> = result.split(";").collect();

        let team1_name = split[0];
        let team2_name = split[1];
        let outcome = split[2];

        // let team1 = teams.entry(team1_name.to_string()).or_insert(Team::new(team1_name.to_string()));
        // let team2 = teams.entry(team2_name.to_string()).or_insert(Team::new(team2_name.to_string()));

        // match outcome {
        //     "win" => {
        //         team1.wins += 1;
        //         team2.loses += 1;
        //     }
        //     "draw" => {
        //         team1.draw += 1;
        //         team2.draw += 1;
        //     }
        //     "loss" => {
        //         team1.loses += 1;
        //         team2.wins += 1;
        //     }
        //     _ => {}
        // }
        // Update team1
        teams
            .entry(team1_name.to_string())
            .and_modify(|team| team.update_stats(outcome, true))
            .or_insert_with(|| {
                let mut team = Team::new(team1_name.to_string());
                team.update_stats(outcome, false);
                team
            });

        // Update team2
        teams
            .entry(team2_name.to_string())
            .and_modify(|team| team.update_stats(outcome, false))
            .or_insert_with(|| {
                let mut team = Team::new(team2_name.to_string());
                team.update_stats(outcome, false);
                team
            });
    }

    let mut result = vec!["Team                           | MP |  W |  D |  L |  P".to_string()];
    // Collect and sort teams
    let mut sorted_teams: Vec<&Team> = teams.values().collect();
    sorted_teams.sort_by(|a, b| {
        b.wins.cmp(&a.wins).then_with(|| a.name.cmp(&b.name)) // Sort by wins, then alphabetically
    });

    for team in sorted_teams {
        result.push(team.clone().to_string());
    }

    result.join("\n")
}
