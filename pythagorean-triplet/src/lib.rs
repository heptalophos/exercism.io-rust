use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::<[u32; 3]>::new();
    let mut a = 3;
    while a <= sum / 3 {
        let b = ((sum * sum) - (2 * sum * a)) / (2 * (sum - a));
        let r = ((sum * sum) - (2 * sum * a)) % (2 * (sum - a));
        let c = sum - (a + b);
        if r == 0 && a < b {
            triplets.insert([a, b, c]);
            a += 2;
        }
        else { a += 1; }
    }
    triplets
}