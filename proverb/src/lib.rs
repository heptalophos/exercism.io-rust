use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
 
    if list.is_empty() {
        return String::new()
    }
    
    let couplets = 
        list.iter().copied()
            .zip(list.iter().copied().skip(1));
    
    let prologue = 
        couplets.map(
            |(premise, conclusion)|
             format!("For want of a {} the {} was lost.",
             premise, conclusion)
            );
    
    let epilogue =
        format!("And all for the want of a {}.", 
                list[0]);

    prologue.chain(once(epilogue)).collect::<Vec<_>>()
            .join("\n")
}