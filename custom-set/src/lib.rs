#[derive(Debug)]

pub struct CustomSet<T: PartialEq + Clone> {
    set: Vec<T>
}

impl<T: PartialEq + Clone> CustomSet<T> {
    pub fn new(elements:  &[T]) -> CustomSet<T> {
        let mut set = CustomSet { set: vec![] };
        for elem in elements {
            set.add(elem.clone())
        }
        set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.set.contains(&element)
    }

    pub fn add(&mut self, element: T) {
        if !self.set.contains(&element) {
            self.set.push(element)
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.set.iter().all(|e| other.set.contains(e))
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.set.iter().any(|e| other.contains(e))
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut common = Self::new(&[]);
        for e in self.set.iter().cloned() {
            if other.contains(&e) { 
                common.add(e) 
            }
        }
        common
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut diff = Self::new(&[]);
        for e in self.set.iter().cloned() {
            if !other.contains(&e) { 
                diff.add(e) 
            }
        }
        diff
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut union = Self::new(&[]);
        for e in self.set.iter()
                         .chain(other.set.iter())
                         .cloned() {
            union.add(e);
        }
        union
    }
}

impl<T: Clone> PartialEq for CustomSet<T> where T: PartialEq {
    fn eq(&self, other: &CustomSet<T>) -> bool {
       self.is_subset(other) && other.is_subset(self)
    }
}