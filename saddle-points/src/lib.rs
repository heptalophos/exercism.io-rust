pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = vec![];
    for (r, row) in input.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            let col: Vec<u64> = input.iter().map(|v| v[c]).collect();
            if row.iter().all(|x| val >= x) && col.iter().all(|x| val <= x){
                saddle_points.push((r, c));
            }
        }
    }
    saddle_points
}
