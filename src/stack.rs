struct Stack {
    stack: Vec<i32>,
    max_size: usize,
}

impl Stack {
    fn new(max_size: usize) -> Stack {
        Stack { 
            stack: Vec::with_capacity(max_size),
            max_size,
        }
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn push(&mut self, value: i32) -> Result<(), &'static str> {
        if self.len() == self.max_size {
            return Err("stack-overflow");
        }
        self.stack.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Result<i32, &'static str> {
        self.stack.pop().ok_or("stack-underflow")
    }

    fn peek(&self) -> Option<&i32> {
        self.stack.last()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_stack() {
        let stack = Stack::new(10);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_push() {
        let mut stack = Stack::new(10);
        assert!(stack.push(1).is_ok());
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_push_overflow() {
        let mut stack = Stack::new(2);
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        assert_eq!(stack.push(3), Err("stack-overflow"));
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new(10);
        stack.push(1).unwrap();
        assert_eq!(stack.pop(), Ok(1));
        assert!(stack.is_empty());
    }

    #[test]
    fn test_pop_underflow() {
        let mut stack = Stack::new(10);
        assert_eq!(stack.pop(), Err("stack-underflow"));
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new(10);
        stack.push(1).unwrap();
        assert_eq!(stack.peek(), Some(&1));
        assert!(!stack.is_empty());
    }
}
