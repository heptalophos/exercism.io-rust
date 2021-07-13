use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], 
                             _second_list: &[T]) -> Comparison {
    use Comparison::*;
    match _first_list.len().cmp(&_second_list.len()) {
        Ordering::Equal 
            if _first_list == _second_list          => Equal,
        Ordering::Less 
            if _first_list.is_empty() || 
               contained(_first_list, _second_list) => Sublist,
        Ordering::Greater 
            if _second_list.is_empty() || 
               contained(_second_list, _first_list) => Superlist,
        _ => Unequal,
    } 
}

fn contained<T: PartialEq>(_alist: &[T], _blist: &[T]) -> bool {
    for (_, list) in _blist.windows(_alist.len()).enumerate() {
        if list == _alist {
            return true
        }
    }
    false
}
