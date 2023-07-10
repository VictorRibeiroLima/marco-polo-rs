pub struct Queue<T> {
    elements: Vec<T>,
    sized: bool,
}

impl<T> Queue<T> {
    /// Creates a new queue.
    pub fn new() -> Self {
        Queue {
            elements: Vec::new(),
            sized: false,
        }
    }

    /// Creates a new queue with a fixed capacity.
    pub fn with_capacity(size: usize) -> Self {
        Queue {
            elements: Vec::with_capacity(size),
            sized: true,
        }
    }

    /// Returns true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Enqueues a value. if the queue is a fixed size and is full, the value is not enqueued.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to enqueue.
    pub fn enqueue(&mut self, value: T) {
        let size = self.elements.len();
        if self.sized && size == self.elements.capacity() {
            return;
        }
        self.elements.push(value);
    }

    /// Dequeues a value. If the queue is empty, None is returned.
    pub fn dequeue(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            return None;
        } else {
            return Some(self.elements.remove(0));
        }
    }

    /// Peeks at the next value of the queue without removing it. If the queue is empty, None is returned.
    pub fn peek(&self) -> Option<&T> {
        if self.elements.is_empty() {
            return None;
        } else {
            return Some(&self.elements[0]);
        }
    }

    /// Returns true if the queue is full.
    pub fn is_full(&self) -> bool {
        let size = self.elements.len();
        if self.sized {
            return size == self.elements.capacity();
        } else {
            return false;
        }
    }

    pub fn len(&self) -> usize {
        return self.elements.len();
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_queue() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_enqueue() {
        let mut queue = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert!(!queue.is_empty());
    }

    #[test]
    fn test_dequeue() {
        let mut queue = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.dequeue(), Some(30));
        assert_eq!(queue.dequeue(), None);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_into_iter() {
        let mut queue = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        let mut iter = queue.into_iter();

        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(30));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_peek() {
        let mut queue = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.peek(), Some(&10));
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.peek(), Some(&20));
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_is_full() {
        let mut queue = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);

        assert_eq!(queue.is_full(), false);

        let mut queue = Queue::with_capacity(2);

        queue.enqueue(10);
        queue.enqueue(20);

        assert_eq!(queue.is_full(), true);
    }

    #[test]
    fn test_enqueue_full() {
        let mut queue = Queue::with_capacity(2);

        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.len(), 2);
        assert_eq!(queue.is_full(), true);
        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.dequeue(), Some(20));
    }
}
