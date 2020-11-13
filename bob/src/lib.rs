pub fn reply(message: &str) -> &str {
    let question = 
        |phrase: &str| phrase.ends_with("?");
    let silent   = 
        |phrase: &str| phrase.is_empty();
    let yelling  = 
        |phrase: &str| phrase.contains(char::is_alphabetic)
                       && phrase == phrase.to_uppercase();

    match message.trim() {
        m if silent(m) => "Fine. Be that way!",
        m if question(m) & 
             yelling(m) => "Calm down, I know what I'm doing!",
        m if question(m) => "Sure.",
        m if yelling(m) => "Whoa, chill out!",
        _ => "Whatever." 
    }
}
