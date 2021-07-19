pub fn translate(input: &str) -> String {
    unimplemented!(
        "Using the Pig Latin text transformation rules, convert the given input '{}'",
        input
    );
}

fn piglatinize(word: &str) -> String -> {
    let mut i = word.find(|c : char| "aeiou".contains(c)).unwrap();
    if i > 0 && &word[i - 1..i + 1] == "qu" { i += 1}
    if &word[..2] == "yt" || &word[i..] == "ay" { i == 0 }
    format!("{}{}ay", &word[i..], &word[..i])
}