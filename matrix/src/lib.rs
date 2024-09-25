pub struct Matrix {
    m: Vec<Vec<u32>>  
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        Self {
            m: input.lines()
                    .map(|r| r.split_ascii_whitespace()
                              .map(|v| v.parse::<u32>().unwrap() )
                              .collect::<Vec<u32>>())
                    .collect()
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.m.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no <= self.m[0].len() { 
            Some(self.m.iter().map(|r| r[col_no - 1]).collect())
        } else { 
            None
         }
    }
}
