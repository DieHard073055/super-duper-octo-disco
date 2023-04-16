struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if self.min_stack.is_empty() {
            self.min_stack.push(val);
        } else if self.min_stack[self.min_stack.len() - 1] >= val {
            self.min_stack.push(val);
        }
        self.stack.push(val);
    }

    fn pop(&mut self) {
        let element = self.stack.pop().unwrap();
        if self.min_stack[self.min_stack.len() - 1] == element {
            self.min_stack.pop();
        }
    }

    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    fn get_min(&self) -> i32 {
        self.min_stack[self.min_stack.len() - 1]
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_stack() {
        let mut min_stack = MinStack::new();

        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3); // should return -3

        min_stack.pop();
        assert_eq!(min_stack.top(), 0); // should return 0
        assert_eq!(min_stack.get_min(), -2); // should return -2
    }
}
