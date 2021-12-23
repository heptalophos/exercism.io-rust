use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;

thread_local!(
    static EXISTING: RefCell<HashSet<String>> = 
        RefCell::new(HashSet::new())
);

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self { name: Robot::generate_name() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::generate_name()
    }

    pub fn generate_name() -> String {
        EXISTING.with(|rs| {
            let mut used_names = rs.borrow_mut();
            loop {
                let new_name = Robot::random_name();
                if used_names.contains(&new_name) { continue; }
                used_names.insert(new_name.clone());
                return new_name;
            }
        })
    }

    fn random_name() -> String {
        let mut rng = rand::thread_rng();
        let mut new_name = String::new();
        for i in 0..=1 {
            new_name.insert(i, rng.gen_range('A'..='Z'));
        }
        for i in 2..=4 {
            new_name.insert(i, rng.gen_range('0'..='9'));
        }
        new_name
    }
}
