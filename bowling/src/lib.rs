#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    rolls: Vec<u16>,
    more_rolls: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: Vec::new(),
            more_rolls: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let last_throw = self.rolls.last();
        if pins > 10 {
            return Err (Error::NotEnoughPinsLeft)
        } else 
        if self.more_rolls && (pins + last_throw.unwrap()) > 10 {
            return Err (Error::NotEnoughPinsLeft)
        } else
        if !self.score().is_none() {
            return Err (Error::GameComplete)
        }
        self.rolls.push(pins);
        self.more_rolls = if pins == 10 { false } else { !self.more_rolls };
        Ok (())
    }

    pub fn score(&self) -> Option<u16> {
        let mut total = 0;
        let mut i = 0;
        let frames = 0..=9;
        for _ in frames {
            if let ( Some(&first), Some(&second) ) = 
                (&self.rolls.get(i), &self.rolls.get(i + 1)) 
            {
                total += first + second;
                if first == 10 || first + second == 10 {
                    if let Some(&over) = &self.rolls.get(i + 2) {
                        total += over;
                    } 
                    else { 
                        return None 
                    }
                }
                i += if first == 10 { 1 } else { 2 };
            } 
            else { 
                return None 
            }
        }
        Some ( total )
    }
}
