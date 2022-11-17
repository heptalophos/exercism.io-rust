use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
    length: usize
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, length: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        let node = Box::new(Node {
            data: element,
            next: self.head.take()
        });
        self.length += 1;
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            let node = *n;
            self.length -= 1;
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();
        while let Some(n) = self.pop() {
            reversed.push(n)
        }
        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iterated = SimpleLinkedList::new();
        for element in iter {
            iterated.push(element)
        }
        iterated
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vector: Vec<T> = Vec::new();
        while let Some(element) = linked_list.pop() {
            vector.push(element)
        }
        vector.reverse();
        vector
    }
}
