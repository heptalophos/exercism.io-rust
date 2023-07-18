pub fn is_armstrong_number(num: u64) -> bool {
       
       num.to_string().chars()
          .fold(0, |mut acc, d| 
           {
              acc += (d.to_digit(10).unwrap() as u64)
                     .pow((((num as f32).log10() as u32) + 1) as u32); 
              acc
           }) 
       == num
}
