pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let countdown = (start_bottles - take_down + 1..=start_bottles).rev();
    countdown.map(|n| {
                let bottlez = bottles(n);
                let this_many = how_many(n);
                format!(
                    "{this_many} green {bottlez} hanging on the wall,\n\
                    {this_many} green {bottlez} hanging on the wall,\n\
                    And if one green bottle should accidentally fall,\n\
                    There'll be {} green {} hanging on the wall.",
                    how_many(n - 1).to_ascii_lowercase(),
                    bottles(n - 1),
                )
             })
             .collect::<Vec<_>>()
             .join("\n\n")
}

fn bottles(n: u32) -> String {
    match n {
        0     => "bottles",
        1     => "bottle",
        2..11 => "bottles",
        _     => unreachable!(),
    }
    .to_string()
}

fn how_many(n: u32) -> String {
    match n {
        0  => "no",
        1  => "One",
        2  => "Two",
        3  => "Three",
        4  => "Four",
        5  => "Five",
        6  => "Six",
        7  => "Seven",
        8  => "Eight",
        9  => "Nine",
        10 => "Ten",
        _  => unreachable!(),
    }
    .to_string()
}
