pub fn verse(n: u32) -> String {
    format!("{p}\n{c}\n", p = premise(n), c = conclusion(n))
}

pub fn sing(start: u32, end: u32) -> String {
    if start == end { 
        return verse(end) 
    } 
    format!("{0}\n{1}", verse(start), sing(start - 1, end).as_str())
}

fn premise(bottles: u32) -> String {
    match bottles {
        0 => format!("{0} {1} {2}", 
                     "No more bottles of beer", 
                     "on the wall,", 
                     "no more bottles of beer."),
        1 => format!("{0} {1}", 
                     "1 bottle of beer on the wall,", 
                     "1 bottle of beer."),
        _ => format!("{b} {bbsw}, {b} {bbs}", 
                     b = bottles, bbs = "bottles of beer.",
                     bbsw = "bottles of beer on the wall")
    }
}

fn conclusion(bottles: u32) -> String {
    match bottles {
        0 => format!("{0} {1}", 
                     "Go to the store and buy some more,",
                     "99 bottles of beer on the wall."),
        1 => format!("{0} {1}", 
                     "Take it down and pass it around,", 
                     "no more bottles of beer on the wall."),
        _ => format!("{0}, {b} bottle{s} {1}",
                     "Take one down and pass it around",
                     "of beer on the wall.",
                     b = bottles - 1, 
                     s = if bottles != 2 { "s" } else { "" })
    }
}