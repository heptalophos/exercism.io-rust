#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {

    let mut input = String::from(input);
    let mut output = String::new();

    while !input.is_empty() {
        output.push(input.pop().unwrap());    
    }
    
    output
}

// and, in order to pass the last (bonus) test 
#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    use unicode_segmentation::UnicodeSegmentation;
    
    input.graphemes(true).rev().collect()
}