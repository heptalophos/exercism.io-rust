use rand::Rng;

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

    fn generate_name() -> String {
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
