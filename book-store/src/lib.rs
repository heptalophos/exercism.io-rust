pub fn lowest_price(books: &[u32]) -> u32 {
    let book_series = vec![1u32, 2u32, 3u32, 4u32, 5u32];
    let mut basket: Vec<_> = 
            book_series.iter()
                 .map(|first_or_sequel| 
                            books
                            .iter()
                            .filter(|&book| book == first_or_sequel)
                            .count())
                 .collect::<Vec<_>>();
    basket.sort_unstable_by(|x,y| y.cmp(x));
    for i in 0..4 { 
        basket[i] -= basket[i + 1]; 
    }
    let pairs3and5 = if basket[2] <= basket[4] {basket[2]} else {basket[4]};                
    basket[2] -= pairs3and5; 
    basket[4] -= pairs3and5; 
    basket[3] += 2 * pairs3and5;
    let discount = vec![0, 5, 10, 20, 25];
    let mut final_price = 0;
    for i in 1..=5 {
        let nr_in_series = basket[i - 1];
        let discount = discount[i - 1];
        final_price += i * nr_in_series * 8 * (100 - discount);
    }
    final_price as u32
}
