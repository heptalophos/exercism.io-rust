#[derive(Debug, PartialEq)]

pub struct CustomSet<T: PartialEq + Clone> {
    set: Vec<T>
}

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn new(elements: Vec<T>) -> Self {
        let mut set = CustomSet {set: vec![]};
        for elem in elements {
            set.add(elem)
        }
        set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.set.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.set.push(element)
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.set.iter().all(|e| other.contains(e))
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        unimplemented!();
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    pub fn difference(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    pub fn union(&self, _other: &Self) -> Self {
        unimplemented!();
    }
}
