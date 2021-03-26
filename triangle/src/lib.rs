use std::collections::HashSet;

pub struct Triangle {
    sides: [u64; 3]
}

impl Triangle {

    // pub fn any_side_zero(&self) -> bool {
    //     if self.sides.iter().any(|&s| s == 0) {
    //         return true
    //     }
    //     false
    // }

    // pub fn triangle_inequality(&self) -> bool {
    //     // let sides = self.sides;
    //     if self.sides.iter().max().unwrap() < 
    //        self.sides.iter().fold(&0u64, |s, &v| {s + v; s})
    //        { return true }
    //     false
    // }

    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let triangle_inequality: bool =
            2u64 * sides.iter().max().unwrap() < 
            sides.iter().sum::<u64>();
        let any_side_zero: bool=
            sides.iter().any(|s| *s == 0);

        if !triangle_inequality || any_side_zero {
            return None;
        }
        Some(Triangle {sides: sides} )
    }

    pub fn distinct_sides(&self) -> u32 {
        let sides = 
            self.sides.iter().copied()
                .collect::<HashSet<u64>>();
        sides.len() as u32
    }

    pub fn is_equilateral(&self) -> bool {
        self.distinct_sides() == 1u32 
    }

    pub fn is_isosceles(&self) -> bool {
        self.distinct_sides() <= 2u32
    }

    pub fn is_scalene(&self) -> bool {
        self.distinct_sides() == 3u32
    }
}
