use std::collections::HashMap;

#[allow(clippy::new_without_default)]
pub struct School {
    roster: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self { roster: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster.entry(grade)
            .or_insert(Vec::new())
            .push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = 
            self.roster.keys().copied()
                .collect::<Vec<u32>>();
        grades.sort_unstable();
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = 
            self.roster.get(&grade).cloned()
                .unwrap_or(Vec::new());
        students.sort_unstable();
        students
    }
}
