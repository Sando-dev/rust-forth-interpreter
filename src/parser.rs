use crate::builtins::Operation;

#[derive(Debug, PartialEq)]
pub enum Token {
    DefStart,    // :
    DefEnd,      // ;
    StringLit(String),
    Number(i16),
    Word(String),  // Para palabras definidas por el usuario
    Operand(Operation), // Para operaciones predefinidas
}


pub fn parse_operation(word: &str) -> Token {
    match word {
        "+" => Token::Operand(Operation::Add),
        "-" => Token::Operand(Operation::Sub),
        "*" => Token::Operand(Operation::Mul),
        "/" => Token::Operand(Operation::Div),
        "DUP" => Token::Operand(Operation::Dup),
        "DROP" => Token::Operand(Operation::Drop),
        "SWAP" => Token::Operand(Operation::Swap),
        "OVER" => Token::Operand(Operation::Over),
        "ROT" => Token::Operand(Operation::Rot),
        "." => Token::Operand(Operation::Dot),
        "EMIT" => Token::Operand(Operation::Emit),
        "CR" => Token::Operand(Operation::Cr),
        "=" => Token::Operand(Operation::Eq),
        "<" => Token::Operand(Operation::Lt),
        ">" => Token::Operand(Operation::Gt),
        "AND" => Token::Operand(Operation::And),
        "OR" => Token::Operand(Operation::Or),
        "NOT" => Token::Operand(Operation::Not),
        "IF" => Token::Operand(Operation::If),
        "ELSE" => Token::Operand(Operation::Else),
        "THEN" => Token::Operand(Operation::Then),
        _ => Token::Word(word.to_string()),
    }
}


fn tokenize(input: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();
    let mut words = input.split_whitespace().peekable();

    while let Some(word) = words.next() {
        match word {
            ":" => tokens.push(Token::DefStart),
            ";" => tokens.push(Token::DefEnd),
            word if word.chars().all(|c| c.is_digit(10) || (c == '-' && word.len() > 1)) => {
                if let Ok(num) = word.parse::<i16>() {
                    tokens.push(Token::Number(num));
                } else {
                    return Err("invalid-number");
                }
            }
            word if word.starts_with('"') => {
                let mut string_content = if word.len() > 1 {
                    word[1..].to_string() 
                } else {
                    String::new()
                };
                
                let mut closed = word.ends_with('"') && word.len() > 1;
                
                while !closed {
                    if let Some(next_word) = words.next() {
                        string_content.push(' ');
                        if next_word.ends_with('"') {
                            string_content.push_str(&next_word[..next_word.len()-1]);
                            closed = true;
                        } else {
                            string_content.push_str(next_word);
                        }
                    } else {
                        return Err("unterminated-string");
                    }
                }
                
                tokens.push(Token::StringLit(string_content));
            }
            _ => tokens.push(parse_operation(&word)), 
        }
    }

    Ok(tokens)
}


pub fn parse(input: &str) -> Result<Vec<Token>, &'static str> {
    let tokens = tokenize(input);
    Ok(tokens?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let result = parse("42").unwrap();
        assert_eq!(result, vec![Token::Number(42)]);
    }

    #[test]
    fn test_parse_operator() {
        let result = parse("+").unwrap();
        assert_eq!(result, vec![Token::Operand(Operation::Add)]);
    }

    #[test]
    fn test_parse_word() {
        let result = parse("FOO").unwrap();
        assert_eq!(result, vec![Token::Word("FOO".to_string())]);
    }

    #[test]
    fn test_parse_definition() {
        let result = parse(": FOO ;").unwrap();
        assert_eq!(result, vec![Token::DefStart, Token::Word("FOO".to_string()), Token::DefEnd]);
    }

    #[test]
    fn test_parse_string_literal() {
        let result = parse(" \"Hello World\" ").unwrap();
        assert_eq!(result, vec![Token::StringLit("Hello World".to_string())]);
    }
}
