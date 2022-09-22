use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    
    let mut triplets = HashSet::<[u32; 3]>::new();
    
    for a in 0..=sum / 3 {
        for b in a + 1..=(sum - a) / 2 {
            let c = sum - (a + b);
            let strict = a < b && b < c;
            let pythagorean = c * c == a * a + b * b;
            if strict && pythagorean {
                triplets.insert([a, b, c]);
            }
        }
    }
    
    triplets
}