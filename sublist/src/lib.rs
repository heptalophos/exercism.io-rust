use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], 
                            second_list: &[T]) -> Comparison {
    use Comparison::*;
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal 
            if first_list == second_list => Equal,
        Ordering::Less 
            if contained(first_list, second_list) => Sublist,
        Ordering::Greater 
            if contained(second_list, first_list) => Superlist,
        _ => Unequal,
    }
}

fn contained<T: PartialEq>(alist: &[T], blist: &[T]) -> bool {
    alist.is_empty() || 
    blist.windows(alist.len())
         .any(|window| window == alist)
}
