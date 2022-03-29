use std::fmt::{Display, Formatter, Result};

pub struct Roman (u32);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        let numeral_map: &[(u32, &str)] = 
            &[ (1, "I"), (4, "IV"), (5, "V"), (9, "IX"),
               (10, "X"), (40, "XL"), (50, "L"), (90, "XC"),
               (100, "C"), (400, "CD"), (50, "D"), (900, "CM"), 
               (1000, "M")
             ];
        let mut arabic = self.0;
        let mut roman = String::new();
        while arabic > 0 {
            let &(n, r) = 
                numeral_map.iter().rev()
                           .filter(|&&(x, _)| x < arabic)
                           .next()
                           .unwrap();
            arabic -= n;
            roman.push_str(r);  
        }
        write!(_f, "{}", roman)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self(num)
    }
}
