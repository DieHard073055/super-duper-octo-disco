use std::collections::VecDeque;

struct MyQueue {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_front(x);
    }

    fn pop(&mut self) -> i32 {
        self.queue.pop_back().unwrap()
    }

    fn peek(&self) -> i32 {
        self.queue.back().unwrap().clone()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_queue() {
        let mut queue = MyQueue::new();

        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.empty(), false);

        queue.push(3);
        assert_eq!(queue.peek(), 2);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.empty(), false);

        assert_eq!(queue.peek(), 3);
        assert_eq!(queue.pop(), 3);
        assert_eq!(queue.empty(), true);
    }
}
