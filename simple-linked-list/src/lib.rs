pub struct SimpleLinkedList<T> {
    list: Vec<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        let list = vec![];
        SimpleLinkedList { list }
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn push(&mut self, element: T) {
        self.list.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.list.last()
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        self.list.reverse();
        self
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let list = iter.into_iter().collect();
        SimpleLinkedList { list }
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        linked_list.list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.
