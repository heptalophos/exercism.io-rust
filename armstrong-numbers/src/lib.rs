pub fn is_armstrong_number(num: u32) -> bool {
    
    num == (0..num.to_string().len() as u32)
           .fold(0, 
                |mut acc, d| 
                {acc += (num/10_u32.pow(d) % 10 )
                        .pow(num.to_string().len() as u32); 
                acc}) 
}
