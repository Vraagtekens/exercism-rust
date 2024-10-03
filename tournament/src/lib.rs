use std::collections::HashMap;
// use std::fmt;

#[derive(Debug, Clone)]
struct Team {
    name: String,
    wins: usize,
    draw: usize,
    loses: usize,
}

impl Team {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            wins: 0,
            draw: 0,
            loses: 0,
        }
    }

    fn to_string(&self) -> String {
        let mp = self.wins + self.draw + self.loses;
        let points = self.wins * 3 + self.draw;

        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name, 
            mp,        
            self.wins,  
            self.draw,  
            self.loses, 
            points      
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

// // Similarly, implement `Display` for `Point2D`.
// impl fmt::Display for Team {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Customize so only `x` and `y` are denoted.
//         // write!(f, "x: {}, y: {}", self.x, self.y)

//         let mp = self.wins + self.draw + self.loses;
//         let points = self.wins * 3 + self.draw;

//         let x = format!(
//             "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
//             self.name, 
//             mp,        
//             self.wins,  
//             self.draw,  
//             self.loses, 
//             points      
//         );

//         write!(f, "{}", x)
//     }
// }

pub fn tally(match_results: &str) -> String {

    if match_results.is_empty() {
        return "Team                           | MP |  W |  D |  L |  P".to_string();
    }

    let mut teams: HashMap<String, Team> = HashMap::new();
    let match_results = match_results.split("\n");

    for result in match_results {
        let split: Vec<&str> = result.split(";").collect();

        let team1_name: &str = split[0];
        let team2_name = split[1];
        let outcome = split[2];

        teams.entry(team1_name.to_string())
            .and_modify(|team| team.update_stats(outcome, true))
            .or_insert_with(|| {
                let mut team = Team::new(team1_name);
                team.update_stats(outcome, true);
                team
            });

        teams.entry(team2_name.to_string())
            .and_modify(|team| team.update_stats(outcome, false))
            .or_insert_with(|| {
                let mut team = Team::new(team2_name);
                team.update_stats(outcome, false);
                team
            });
    }

    let mut result = vec!["Team                           | MP |  W |  D |  L |  P".to_string()];
    // Collect and sort teams
    let mut sorted_teams: Vec<&Team> = teams.values().collect();
    sorted_teams.sort_by(|a, b| {

        let a_points = a.wins * 3 + a.draw;
        let b_points = b.wins * 3 + b.draw;

        b_points.cmp(&a_points).then(
            b.wins.cmp(&a.wins).then_with(|| a.name.cmp(&b.name)) // Sort by wins, then alphabetically
        )
    });

    for team in sorted_teams {
        let team_name = team.clone().to_string();
        result.push(team_name);
    }


    result.join("\n")
}
