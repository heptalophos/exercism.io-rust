pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    items
    .iter().enumerate()
    .filter_map(|(i, item)| {
        if item.weight > max_weight { None }
        else {
            let new_capacity = max_weight - item.weight;
            let taken = item.value;
            let not_yet_taken = &items[(i + 1)..];
            Some (taken + maximum_value(new_capacity, not_yet_taken))
        }
    })
    .max()
    .unwrap_or_default()
}