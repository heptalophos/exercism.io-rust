use std::collections::HashSet;

pub struct Triangle {
    sides: [u64; 3]
}

impl Triangle {

    // pub fn any_side_zero(sides: [u64; 3]) -> bool {
    //     sides.iter().any(|&s| s == 0)
    // }

    // pub fn triangle_inequality(sides: [u64; 3]) -> bool {
    //     sides.iter().max().unwrap() 
    //     < 
    //     sides.iter().fold(&0u64, |s, &v| {s + v; s})
    // }

    pub fn distinct_sides(&self) -> usize {
        self.sides
            .iter()
            .cloned()
            .collect::<HashSet<u64>>()
            .len()
    }

    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let triangle_inequality : bool = 
            sides.iter().max().unwrap() < 
            sides.iter().fold(&0u64, |s, &v| {(*s + &v); s});
            // &sides.iter().sum::<u64>();
        let any_side_zero : bool = 
            sides.iter().any(|&s| s == 0);

        if !triangle_inequality || any_side_zero {
            return None;
        }
        Some(Triangle {sides} )
    }

    pub fn is_equilateral(&self) -> bool {
        return &self.distinct_sides() == &1usize; 
    }

    pub fn is_scalene(&self) -> bool {
        return &self.distinct_sides() == &2usize;
    }

    pub fn is_isosceles(&self) -> bool {
        return &self.distinct_sides() == &3usize
    }
}
