pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    max_value(max_weight, 0, &items.iter().collect::<Vec<_>>())
}

fn max_value(max_weight: u32, tot_value: u32, items: &[&Item]) -> u32 {
    items.iter()
         .enumerate()
         .filter_map(|(i, item)| {
            if item.weight > max_weight {
                Some (tot_value)
            }
            else { 
            let mut remaining = items.to_vec();
            remaining.remove(i);
            Some (max_value(max_weight - item.weight, 
                            tot_value + item.value, 
                            &remaining))
            }
          })
         .max()
         .unwrap_or_default()
}