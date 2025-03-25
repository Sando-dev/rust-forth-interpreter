#[derive(Debug, PartialEq)]
enum Token {
    Number(i16),
    Word(String),
}


fn tokenize(input: &str) -> Vec<Token> {
    input.split_whitespace().map(|word| {
        if let Ok(num) = word.parse::<i16>() {
            Token::Number(num)
        } else {
            Token::Word(word.to_uppercase()) 
        }
    }).collect()
}


fn main() {
    let input = "25 10 + 3 * á .";
    let tokens = tokenize(input);
    println!("{:?}", tokens);
}