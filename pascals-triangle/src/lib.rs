pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        
        let mut ptriangle = PascalsTriangle {
            rows: Vec::new(),
        };
        
        for n in 0..row_count {
            let mut row = Vec::new();
            for k in 0..(n + 1) {
                row.push(binomial(n, k))
            }
            ptriangle.rows.push(row);
        }
        
        ptriangle
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.to_vec()
    }
}

fn binomial(n: u32, k: u32) -> u32 {
    if k > n { return 0 } 
    if k == 0 || k == n { return 1 }
    binomial(n - 1, k - 1) + binomial(n - 1, k) 
}
