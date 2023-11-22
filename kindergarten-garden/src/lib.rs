pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let child_cups = (2 * (student.bytes().nth(0).unwrap() - b'A')).into();
    diagram
    .lines()
    .flat_map(
        |line| {
            line[child_cups..=child_cups + 1]
            .chars()
            .map(|p| match p {
                    'G' => "grass", 
                    'C' => "clover", 
                    'R' => "radishes", 
                    'V' => "violets",
                    _ => panic!("invalid cup"),
                })
            })
    .collect()
}