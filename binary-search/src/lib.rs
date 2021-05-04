pub fn find(array: &[i32], key: i32) -> Option<usize> {
    
    let haystaq = array.as_ref();
    let mut min = 0;
    let mut max = haystaq.len();

    while min < max {
        let mid = min / 2 + max / 2;
        if key < haystaq[mid] { max = mid }
        else if key > haystaq[mid] { min = mid + 1 }
        else { return Some(mid) }
    }
    None
}
