#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr,)*) => {
        {   
            let mut hash_map = macros::hashmap!();
            $(hash_map.insert($key, $value);)*
            hash_map
        }
    };
    ($($key:expr => $value:expr),*) => {
        macros::hashmap!($($key => $value,)*)
    };
}
