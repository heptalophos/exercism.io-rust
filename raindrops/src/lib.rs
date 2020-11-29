pub fn raindrops(n: u32) -> String {
    
    let mut sounds = 
        String::new();

    if n % 3 == 0 {
        sounds += "Pling"
    };
    if n % 5 == 0 {
        sounds += "Plang"
    };
    if n % 7 == 0 {
        sounds += "Plong"
    };

    if sounds == "" { 
        n.to_string() 
    } 
    else { 
        sounds 
    } 
}
