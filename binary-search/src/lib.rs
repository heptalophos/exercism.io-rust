pub fn find<H, N>(array: H, key: N) -> Option<usize> 
where H: AsRef<[N]>, N: Ord {
    use std::cmp::Ordering::{Less, Equal, Greater};

    let haystaq = array.as_ref();
    let mut min = 0;
    let mut max = haystaq.len();

    while min <= max {
        
        if min > max { return None }
        
        let mid = (min + max) / 2;
        let needle = key.cmp(haystaq.get(mid)?);
        
        match needle {
            Less => {
                if mid > 0 { max = mid - 1 }
                else { return None }
            },
            Greater => min = mid + 1,
            Equal => return Some(mid)
        }
    }
    
    None
}
