pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.iter().any(|r| r.len() != input[0].len()) {
        panic!("not a matrix");
    }
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();
    for (r, row) in input.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            let col = input.iter().map(|rx| rx[c]).collect::<Vec<u64>>();
            if row.iter().all(|rx| val >= rx) && col.iter().all(|cx| val <= cx) {
                saddle_points.push((r, c));
            }
        }
    }
    saddle_points
}
