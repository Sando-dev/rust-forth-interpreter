use crate::stack::Stack;
use std::io::{self, Write}; 


#[derive(Debug, PartialEq)]
pub enum Operation {
    // Aritmética
    Add, Sub, Mul, Div,
    
    // Manipulación de stack
    Dup, Drop, Swap, Over, Rot,

    // Output
    Dot, Emit, Cr, //PrintString,

    // Booleanas
    Eq, Lt, Gt, And, Or, Not,

    // Condicionales
    If, Else, Then,
}

impl Operation {
    pub fn execute(&self, stack: &mut Stack) -> Result<(), &'static str> {
        match self {
            //Aritmeticas
            Self::Add => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a + b)?;
            }

            Self::Sub => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b - a)?;
            }

            Self::Mul => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a * b)?;
            }   

            Self::Div => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                if a == 0 {
                    return Err("division-by-zero");
                }
                stack.push(b / a)?;
            }

            // Manipulacion de stack
            Self::Dup => {
                let a = stack.pop()?;
                stack.push(a)?;
                stack.push(a)?;
            }

            Self::Drop => {
                stack.pop()?;
            }

            Self::Swap => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a)?;
                stack.push(b)?;
            }

            Self::Over => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b)?;
                stack.push(a)?;
                stack.push(b)?;
            }

            Self::Rot => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                let c = stack.pop()?;
                stack.push(b)?;
                stack.push(a)?;
                stack.push(c)?;
            }

            // Output
            Self::Dot => {
                let a = stack.pop()?;
                print!("{} ", a); 
                io::stdout().flush().map_err(|_| "flush failed")?; // Fuerza la salida inmediata
            }

            Self::Emit => {
                let a = stack.pop()?;
                print!("{}", a as u8 as char);
            }

            Self::Cr => {
                println!();
            }

            // Booleanas
            Self::Eq => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(if a == b { 1 } else { 0 })?;
            }

            Self::Lt => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(if b < a { 1 } else { 0 })?;
            }

            Self::Gt => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(if b > a { 1 } else { 0 })?;
            }

            Self::And => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(if a != 0 && b != 0 { 1 } else { 0 })?;
            }   

            Self::Or => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(if a != 0 || b != 0 { 1 } else { 0 })?;
            }

            Self::Not => {
                let a = stack.pop()?;
                stack.push(if a == 0 { 1 } else { 0 })?;
            }

            // Condicionales
            Self::If | Self::Else | Self::Then => {
                return Err("Conditionals should be handled by the interpreter");
            }

        }
        Ok(())
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::Stack;

    #[test]
    fn test_add_operation() {
        let mut stack = Stack::default();
        stack.push(10).unwrap();
        stack.push(20).unwrap();
        
        // Ejecutar ADD
        Operation::Add.execute(&mut stack).unwrap();
        
        // Verificar resultado
        assert_eq!(stack.pop().unwrap(), 30);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_add_with_underflow() {
        let mut stack = Stack::default();
        stack.push(5).unwrap();
        
        // Intentar ADD con stack insuficiente
        let result = Operation::Add.execute(&mut stack);
        assert_eq!(result, Err("stack-underflow"));
    }

    #[test]
    fn test_swap_operation() {
        let mut stack = Stack::default();
        stack.push(10).unwrap();
        stack.push(20).unwrap();
        
        // Ejecutar SWAP
        Operation::Swap.execute(&mut stack).unwrap();
        
        // Verificar orden invertido
        assert_eq!(stack.pop().unwrap(), 10);
        assert_eq!(stack.pop().unwrap(), 20);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_swap_with_insufficient_items() {
        let mut stack = Stack::default();
        stack.push(5).unwrap();
        
        // Intentar SWAP con solo un elemento
        let result = Operation::Swap.execute(&mut stack);
        assert_eq!(result, Err("stack-underflow"));
    }
}