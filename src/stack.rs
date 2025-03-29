pub struct Stack {
    stack: Vec<i16>,
    max_size: usize,
}

impl Stack {
    pub fn new(max_size: usize) -> Stack {
        Stack { 
            stack: Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn default() -> Self {
        Self::new(128 * 1024 / std::mem::size_of::<i16>()) // 128 KB en i16
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn push(&mut self, value: i16) -> Result<(), &'static str> {
        if self.len() == self.max_size {
            return Err("stack-overflow");
        }
        self.stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<i16, &'static str> {
        self.stack.pop().ok_or("stack-underflow")
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn get_contents(&mut self) -> Vec<i16> {
        std::mem::take(&mut self.stack) 
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

}
