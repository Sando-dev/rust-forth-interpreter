#[derive(Debug, PartialEq)]
enum Token {
    DefStart, // :
    DefEnd, // ;
    StringLit(String),
    Number(i32),
    Word(String),
}



fn tokenize(input: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();
    let mut words = input.split_whitespace().peekable();

    while let Some(word) = words.next() {
        match word {
            ":" => tokens.push(Token::DefStart),
            ";" => tokens.push(Token::DefEnd),
            word if word.chars().all(|c| c.is_digit(10) || (c == '-' && word.len() > 1)) => {
                if let Ok(num) = word.parse::<i32>() {
                    tokens.push(Token::Number(num));
                } else {
                    return Err("invalid-number");
                }
            }
            r#"""# => {
                let mut string_content = String::new();
                let mut string_closed = false;

                while let Some(next_word) = words.next() {
                    if next_word.ends_with('"') {
                        string_content.push_str(&next_word[..next_word.len() - 1]);
                        string_closed = true;
                        break;
                    } else {
                        string_content.push_str(next_word);
                        string_content.push(' ');
                    }
                }

                if !string_closed {
                    return Err("unterminated-string");
                }

                // Pasamos la string a una variable temporal antes de moverla al Token
                let final_string = string_content;
                tokens.push(Token::StringLit(final_string));
            }
            _ => tokens.push(Token::Word(word.to_uppercase())),
        }
    }

    Ok(tokens)
}




fn parse_words(tokens: Vec<Token>) -> Result<Vec<Token>, &'static str> {
    let mut parsed_tokens = Vec::new();
    let mut defining_word = false;
    let mut word_name = None;
    let mut word_body = Vec::new();

    let mut iter = tokens.into_iter(); 

    while let Some(token) = iter.next() {
        match token {
            Token::DefStart => {
                if defining_word {
                    return Err("nested-word-definition");
                }
                defining_word = true;
            }
            Token::DefEnd => {
                if !defining_word {
                    return Err("unexpected-;");
                }
                defining_word = false;
                if let Some(name) = word_name.take() {
                    parsed_tokens.push(Token::Word(format!("DEFINE_{}", name)));
                    parsed_tokens.append(&mut word_body);
                } else {
                    return Err("invalid-word");
                }
            }
            Token::Number(_) if defining_word && word_name.is_none() => {
                return Err("invalid-word"); // No se permite definir una word con número
            }
            Token::Word(w) if defining_word => {
                if word_name.is_none() {
                    word_name = Some(w);
                } else {
                    word_body.push(Token::Word(w));
                }
            }
            _ if defining_word => {
                word_body.push(token);
            }
            _ => {
                parsed_tokens.push(token);
            }
        }
    }

    if defining_word {
        return Err("unterminated-word-definition"); // Falta el `;`
    }

    Ok(parsed_tokens)
}

fn parse_conditionals(tokens: Vec<Token>) -> Result<Vec<Token>, &'static str> {
    let mut parsed = Vec::new();
    let mut stack = Vec::new(); // Almacena posiciones de IF/ELSE

    for token in tokens {
        match token {
            Token::Word(w) if w == "IF" => {
                stack.push((parsed.len(), false)); // (position, has_else)
                parsed.push(Token::Word("IF".to_string()));
            }
            Token::Word(w) if w == "ELSE" => {
                if let Some((if_pos, has_else)) = stack.last_mut() {
                    if *has_else {
                        return Err("duplicate-else");
                    }
                    *has_else = true;
                    parsed[*if_pos] = Token::Word("IF_ELSE".to_string());
                    parsed.push(Token::Word("ELSE".to_string()));
                } else {
                    return Err("unexpected-else");
                }
            }
            Token::Word(w) if w == "THEN" => {
                if let Some((_, has_else)) = stack.pop() {
                    parsed.push(Token::Word("THEN".to_string()));
                } else {
                    return Err("unexpected-then");
                }
            }
            _ => parsed.push(token),
        }
    }

    if !stack.is_empty() {
        return Err("unterminated-if");
    }

    Ok(parsed)
}


fn parse(input: &str) -> Result<Vec<Token>, &'static str> {
    let tokens = tokenize(input);
    let tokens = parse_words(tokens?)?;
    let tokens = parse_conditionals(tokens)?;
    Ok(tokens)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic_words() {
        let input = "DUP SWAP DROP";
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Word("DUP".to_string()),
                Token::Word("SWAP".to_string()),
                Token::Word("DROP".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_numbers() {
        let input = "42 -5 1000";
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(42),
                Token::Number(-5),
                Token::Number(1000),
            ]
        );
    }

    #[test]
    fn test_tokenize_definition_tokens() {
        let input = ": NEGATE -1 * ;";
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::DefStart,
                Token::Word("NEGATE".to_string()),
                Token::Number(-1),
                Token::Word("*".to_string()),
                Token::DefEnd,
            ]
        );
    }

    #[test]
    fn test_tokenize_string_literal() {
        let input = "\" Hello, World!\" ";
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![Token::StringLit("Hello, World!".to_string())]
        );
    }

    #[test]
    fn test_tokenize_unterminated_string() {
        let input = "\" Hello, World!";
        let result = tokenize(input);
        assert_eq!(result, Err("unterminated-string"));
    }

    #[test]
    fn test_parse_words_simple_definition() {
        let tokens = vec![
            Token::DefStart,
            Token::Word("NEGATE".to_string()),
            Token::Number(-1),
            Token::Word("*".to_string()),
            Token::DefEnd,
        ];
        
        let parsed = parse_words(tokens).unwrap();
        assert_eq!(
            parsed,
            vec![
                Token::Word("DEFINE_NEGATE".to_string()),
                Token::Number(-1),
                Token::Word("*".to_string()),
            ]
        );
    }

    #[test]
    fn test_parse_words_nested_definition_error() {
        let tokens = vec![
            Token::DefStart,
            Token::Word("NEGATE".to_string()),
            Token::DefStart,
            Token::Word("OTHER".to_string()),
            Token::DefEnd,
            Token::DefEnd,
        ];

        let result = parse_words(tokens);
        assert_eq!(result, Err("nested-word-definition"));
    }

    #[test]
    fn test_parse_words_invalid_word() {
        let tokens = vec![
            Token::DefStart,
            Token::Number(1),  // Esto no es válido
            Token::Word("SOMETHING".to_string()),
            Token::DefEnd,
        ];

        let result = parse_words(tokens);
        assert_eq!(result, Err("invalid-word"));
    }

    #[test]
    fn test_parse_words_unterminated_definition() {
        let tokens = vec![
            Token::DefStart,
            Token::Word("NEGATE".to_string()),
            Token::Number(-1),
            Token::Word("*".to_string()),
        ];

        let result = parse_words(tokens);
        assert_eq!(result, Err("unterminated-word-definition"));
    }

    #[test]
    fn test_parse_conditionals_simple_if() {
        let tokens = vec![
            Token::Word("IF".to_string()),
            Token::Number(1),
            Token::Word("<".to_string()),
            Token::Word("THEN".to_string()),
        ];
        
        let parsed = parse_conditionals(tokens).unwrap();
        assert_eq!(
            parsed,
            vec![
                Token::Word("IF".to_string()),
                Token::Number(1),
                Token::Word("<".to_string()),
                Token::Word("THEN".to_string()),
            ]
        );
    }

    #[test]
    fn test_parse_conditionals_if_else() {
        let tokens = vec![
            Token::Word("IF".to_string()),
            Token::Number(1),
            Token::Word("<".to_string()),
            Token::Word("ELSE".to_string()),
            Token::Number(2),
            Token::Word("THEN".to_string()),
        ];
        
        let parsed = parse_conditionals(tokens).unwrap();
        assert_eq!(
            parsed,
            vec![
                Token::Word("IF_ELSE".to_string()),
                Token::Number(1),
                Token::Word("<".to_string()),
                Token::Word("ELSE".to_string()),
                Token::Number(2),
                Token::Word("THEN".to_string()),
            ]
        );
    }

    #[test]
    fn test_parse_conditionals_unexpected_else() {
        let tokens = vec![Token::Word("ELSE".to_string())];
        let result = parse_conditionals(tokens);
        assert_eq!(result, Err("unexpected-else"));
    }

    #[test]
    fn test_parse_conditionals_unexpected_then() {
        let tokens = vec![Token::Word("THEN".to_string())];
        let result = parse_conditionals(tokens);
        assert_eq!(result, Err("unexpected-then"));
    }

    #[test]
    fn test_parse_conditionals_unterminated_if() {
        let tokens = vec![
            Token::Word("IF".to_string()),
            Token::Number(1),
            Token::Word("<".to_string()),
        ];

        let result = parse_conditionals(tokens);
        assert_eq!(result, Err("unterminated-if"));
    }

}


