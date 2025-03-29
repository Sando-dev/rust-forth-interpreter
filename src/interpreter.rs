use crate::parser::{Token, parse};
use crate::stack::Stack;

pub struct Interpreter {
    stack: Stack,
}

impl Interpreter {
    /// Crea un nuevo intérprete con un tamaño de stack opcional
    pub fn new(stack_size: Option<usize>) -> Self {
        Self {
            stack: Stack::new(stack_size.unwrap_or(128 * 1024 / std::mem::size_of::<i16>())),
        }
    }

    /// Ejecuta un programa en Forth
    pub fn run(&mut self, input: &str) -> Result<(), &'static str> {
        let tokens = parse(input)?;
        self.execute(tokens)
    }

    /// Ejecuta una lista de tokens
    fn execute(&mut self, tokens: Vec<Token>) -> Result<(), &'static str> {
        let mut iter = tokens.into_iter();
        while let Some(token) = iter.next() {
            match token {
                Token::Number(num) => self.stack.push(num)?,  
                Token::Operand(op) => op.execute(&mut self.stack)?, 
                Token::Word(_word) => self.stack.push(2)?, //Cambiar despues
                Token::DefStart => self.stack.push(1)?, //Cambiar despues
                _ => return Err("Unexpected token"),
            }
        }
        Ok(())
    }

    pub fn get_stack_contents(&mut self) -> Vec<i16> {
        self.stack.get_contents()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter_basic_operations() {
        // Crear intérprete con stack pequeño para testing (10 elementos)
        let mut interpreter = Interpreter::new(Some(10));

        // Test 1: Push números y suma
        let result = interpreter.run("10 20 +");
        assert!(result.is_ok());
        assert_eq!(interpreter.stack.pop().unwrap(), 30);

        // Test 2: Operación con underflow
        let result = interpreter.run("+");
        assert_eq!(result, Err("stack-underflow"));

        // Test 3: Multiplicación
        interpreter.run("3 4 *").unwrap();
        assert_eq!(interpreter.stack.pop().unwrap(), 12);
    }

    #[test]
    fn test_stack_size_limits() {
        // Intérprete con stack de solo 2 elementos
        let mut interpreter = Interpreter::new(Some(2));

        // Llenar el stack
        assert!(interpreter.run("1 2").is_ok());

        // Debería fallar al intentar poner un tercer elemento
        let result = interpreter.run("3");
        assert_eq!(result, Err("stack-overflow"));
    }
}