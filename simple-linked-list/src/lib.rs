use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: Option::None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut current = &self.head;
        let mut counter: usize = 0;
        while let Option::Some(n) = current {
            counter += 1;
            current = &n.previous;
        }
        counter
    }

    pub fn push(&mut self, _element: T) {
        self.head = Option::Some(Box::new(Node::new(_element, self.head.take())));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(n) => {
                self.head = n.previous;
                return Option::Some(n.value);
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head.as_ref() {
            None => None,
            Some(n) => Option::Some(&n.value),
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut ret = SimpleLinkedList::new();
        while let Some(x) = self.pop() {
            ret.push(x);
        }
        ret
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut ll = SimpleLinkedList::new();
        _iter.into_iter().for_each(|val| ll.push(val));
        ll
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut to_return = Vec::new();
        let mut current = _linked_list.head.unwrap();
        while current.previous.is_some() {
            to_return.push(current.value);
            current = current.previous.unwrap();
        }
        to_return.reverse();
        to_return
    }
}

pub struct Node<T> {
    value: T,
    previous: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, previous: Option<Box<Node<T>>>) -> Self {
        Node { value, previous }
    }
}