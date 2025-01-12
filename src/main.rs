mod lexer;

fn main() {
    let tokens = lexer::tokenize(String::from("\'test\' \'another one ef324f839gneg4r^\'"));
    for token in &tokens {
        match token {
            lexer::Token::String(s) => println!("{}", s),
            _ => {}
        }
    }
}
