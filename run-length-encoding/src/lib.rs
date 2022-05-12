use itertools::Itertools;

pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    for (ch, group) in source.chars().group_by(|c| *c).into_iter() 
    {
        match group.count() {
            1 => encoded.push_str(format!("{}", ch).as_str()),
            n => encoded.push_str(format!("{}{}", n, ch).as_str())
        }                            
    }
    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    for (n, c) in source.split(|c: char| !&c.is_numeric())
                        .map(|n| n.parse().unwrap_or(1))
                        .zip(source.matches(|c: char| !&c.is_numeric()))
    {
        decoded.push_str(format!("{}", c.repeat(n)).as_str())
    }
    decoded
}
