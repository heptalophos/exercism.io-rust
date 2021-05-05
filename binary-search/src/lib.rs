pub fn find<H, N>(array: H, key: N) -> Option<usize> 
where H: AsRef<[N]>, N: Ord {
    use std::cmp::Ordering;

    let haystaq = array.as_ref();
    let (mut min, mut max) = (0, haystaq.len());

    while min <= max {
        
        if min > max { return None }
        
        let mid = (min + max) / 2;
        let needle = key.cmp(haystaq.get(mid)?);
        
        match needle {
            Ordering::Less => {
                if mid > 0 { max = mid - 1 }
                else { return None }
            },
            Ordering::Greater => min = mid + 1,
            Ordering::Equal => return Some(mid)
        }
    }
    
    None
}
