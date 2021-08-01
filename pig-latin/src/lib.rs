pub fn translate(input: &str) -> String {
    input.split_ascii_whitespace()
         .map(piglatinize)
         .collect::<Vec<_>>()
         .join(" ")
}

fn piglatinize(word: &str) -> String {
    let mut p : usize = 
        word.find(|c : char| "aeiouy".contains(c))
            .unwrap();
    let rule1 : bool = p == 0 && &word[..1] == "y";
    let rule2 : bool = p > 0 && &word[p - 1..p + 1] == "qu";
    let rule3 : bool = &word[..2] == "yt";
    let rule4 : bool = &word[p..] == "ay";
    if  rule1 || rule2 { p += 1 }
    if  rule3 || rule4 { p = 0 }
    format!("{}{}ay", &word[p..], &word[..p])
}
