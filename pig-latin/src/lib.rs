pub fn translate(input: &str) -> String {
    input.split_ascii_whitespace()
         .map(piglatinize)
         .collect::<Vec<_>>()
         .join(" ")
}

fn piglatinize(word: &str) -> String {
    let mut i : usize = 
        word.find(|c : char| "aeiouy".contains(c))
            .unwrap();
    let rule1 : bool = i == 0 && & word[..1] == "y";
    let rule2 : bool = i > 0 && &word[i - 1..i + 1] == "qu";
    let rule3 : bool = &word[..2] == "yt";
    let rule4 : bool = &word[i..] == "ay";
    if  rule1 || rule2 { i += 1 }
    if  rule3 || rule4 { i = 0 }
    format!("{}{}ay", &word[i..], &word[..i])
}
