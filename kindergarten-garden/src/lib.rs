pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let child_plants = (2 * (student.bytes().next().unwrap() - b'A')).into();
    diagram
    .lines()
    .flat_map(
        |line| {
            line[child_plants..=child_plants + 1]
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