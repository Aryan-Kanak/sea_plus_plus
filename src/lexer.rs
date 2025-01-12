pub enum Token {
    EOF,
    Identifier(String),
    String(String),
    Number(String),
    False,
    True,
    If,
    Else,
    EndIf,
    While,
    EndWhile,
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Equal,
    GreaterThan,
    LessThan,
    Not,
    Or,
    And,
    DeclareMethod,
    MethodArguments,
    Return,
    EndDeclareMethod,
    CallMethod,
    AssignVariable,
    EndAssignVariable,
    DeclareInt,
    DeclareBool,
    DeclareString,
    SetInitialValue,
    BeginMain,
    EndMain,
    Print
}

pub fn tokenize(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut it= source.chars().peekable();
    while let Some(c) = it.next() {
        match c {
            c if c.is_whitespace() => continue,
            '\'' => {
                let mut str: String = String::new();
                while it.peek().is_some()
                {
                    let c: char = it.next().unwrap();
                    if c == '\'' {
                        tokens.push(Token::String(str));
                        break;
                    } else {
                        str.push(c);
                    }
                }
            },
            _ => panic!("Unrecognized character")
        }
    }

    tokens
}
