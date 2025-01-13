mod lexer;

fn main() {
    let tokens = lexer::tokenize(String::from("first se2_cond  22131 \'test\' \'another one ef324f839gneg4r^\'"));
    for token in &tokens {
        match token {
            lexer::Token::String(s) => println!("String: {}", s),
            lexer::Token::Identifier(s) => println!("Identifier: {}", s),
            lexer::Token::Number(s) => println!("Number: {}", s),
            _ => {}
        }
    }
}
