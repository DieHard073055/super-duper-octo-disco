struct MyQueue {
    push_stack: Vec<i32>,
    pop_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            push_stack: Vec::new(),
            pop_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.push_stack.push(x);
    }
    fn pop(&mut self) -> i32 {
        if self.pop_stack.is_empty() {
            while let Some(x) = self.push_stack.pop() {
                self.pop_stack.push(x);
            }
        }
        self.pop_stack.pop().unwrap()
    }

    fn peek(&self) -> i32 {
        if !self.pop_stack.is_empty() {
            self.pop_stack[self.pop_stack.len() - 1]
        } else {
            self.push_stack[0]
        }
    }

    fn empty(&self) -> bool {
        if !self.pop_stack.is_empty() {
            false
        } else if !self.push_stack.is_empty() {
            false
        } else {
            true
        }
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
