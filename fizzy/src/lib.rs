use std::ops::Rem;

type Predicate<T> = Box<dyn Fn(T) -> bool>;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
/// 
pub struct Matcher<T>(Predicate<T>, String);

impl<T> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static, S: AsRef<str>>(matcher: F, subs: S) -> Matcher<T> 
    {
        Self(Box::new(matcher) as Predicate<T>, subs.as_ref().to_string())
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
pub struct Fizzy<T>(Vec<Matcher<T>>);

impl<T: Copy + ToString> Fizzy<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.0.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    /// 
    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map(move |item| self.fizzy(item))
    }

    fn fizzy(&self, item: T) -> String {
        let sub = self.0.iter()
                  .filter(|m| (m.0)(item))
                  .map(|m| m.1.clone())
                  .collect::<String>();
        if sub.is_empty() {
            item.to_string()
        }
        else {
            sub
        }
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
/// 
pub fn fizz_buzz<T: Rem<Output = T> + Copy + ToString + From<u8> + PartialEq>() -> Fizzy<T> {
    Fizzy::new()
    .add_matcher(Matcher::new(|x: T| x % T::from(3) == T::from(0), "fizz"))
    .add_matcher(Matcher::new(|x: T| x % T::from(5) == T::from(0), "buzz"))
}
