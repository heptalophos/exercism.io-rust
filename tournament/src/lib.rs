use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct TeamRecord<'a> {
    name: &'a str,
    matches: usize,
    wins: usize,
    draws: usize,
    losses: usize,
    points: usize,
}

impl<'a> TeamRecord<'a> {
    pub fn new(name: &'a str) -> Self {
        TeamRecord 
        { 
            name, 
            matches: 0, 
            wins: 0, 
            draws: 0, 
            losses: 0, 
            points: 0, 
        }
    }
    pub fn won(&mut self) {
        self.matches += 1;
        self.wins += 1;
        self.points += 3;
    }
    pub fn drawn(&mut self) {
        self.matches += 1;
        self.draws += 1;
        self.points += 1;
    }
    pub fn lost(&mut self) {
        self.matches += 1;
        self.losses += 1;
    }
    pub fn to_string(&self) -> String {
        format!("{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
                self.name, self.matches, self.wins, 
                self.draws, self.losses, self.points.to_string())
    }
}

pub fn tally(match_results: &str) -> String {
    let mut tournament: HashMap<&str, TeamRecord> = HashMap::new();
    for line in match_results.lines() {
        let fields: Vec<&str> = line.split(";").collect();
        let home: &str = fields[0];
        let away: &str = fields[1];
        let outcome = fields[2];
        match outcome {
            "win" => {
                tournament.entry(home)
                          .or_insert(TeamRecord::new(home))
                          .won();
                tournament.entry(away)
                          .or_insert(TeamRecord::new(away))
                          .lost();
            }
            "draw" => {
                tournament.entry(home)
                        .or_insert(TeamRecord::new(home))
                        .drawn();
                tournament.entry(away)
                        .or_insert(TeamRecord::new(away))
                        .drawn();
            }
            "loss" => {
                tournament.entry(home)
                          .or_insert(TeamRecord::new(home))
                          .lost();
                tournament.entry(away)
                          .or_insert(TeamRecord::new(away))
                          .won();
            }
            _ => panic!("Invalid match results.")
        }
    }
    let mut sorted = tournament.values().collect::<Vec<_>>();
    sorted.sort_by_key(|&team| (0 - (team.points as i32), &team.name));

    let mut table = vec![format!("{:31}| MP |  W |  D |  L |  P", "Team")];
    for t in sorted.iter() {
        table.push(t.to_string());
    }
    table.join("\n")
}
