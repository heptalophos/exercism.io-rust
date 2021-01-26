#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: Vec::from(scores),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores[..]
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last()
                   .copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter()
                   .max()
                   .copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut cscores = 
            self.scores.clone();
        cscores.sort();
        cscores.iter()
               .rev()
               .take(3)
               .copied()
               .collect()
    }
}
