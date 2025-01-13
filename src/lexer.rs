use std::{collections::HashMap, iter::Peekable};
use phf::phf_map;

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

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "This is a load of barnacles" => Token::False,
    "Correct!" => Token::True,
    "Is this the Krusty Krab" => Token::If,
    "No this is Patrick" => Token::Else,
    "Alright pinhead, your time is up" => Token::EndIf,
    "I'm ready!" => Token::While,
    "We do that for forty years and then we die" => Token::EndWhile,
    "You know what's funnier than twenty-four" => Token::Plus,
    "Good grief he's naked!" => Token::Minus,
    "Maximum overdrive" => Token::Mult,
    "This is advanced darkness" => Token::Div,
    "The inner machinations of my mind are an enigma" => Token::Mod,
    "Is mayonnaise an instrument?" => Token::Equal,
    "Futureee" => Token::GreaterThan,
    "Don't touch me I'm sterile" => Token::LessThan,
    "Oh no he's hot" => Token::Not,
    "Are you feeling it now Mr. Krabs" => Token::Or,
    "It's not just a boulder, it's a rock" => Token::And,
    "Listen you crustaceous cheapskate!" => Token::DeclareMethod,
    "I neeeeed it" => Token::MethodArguments,
    "Goodbye everyone, I'll remember you all in therapy" => Token::Return,
    "My leg!" => Token::EndDeclareMethod,
    "Rev up those fryers" => Token::CallMethod,
    "I will now assault your mind with subliminal messages" => Token::AssignVariable,
    "Firmly grasp it" => Token::EndAssignVariable,
    "I'm a goofy goober" => Token::DeclareInt,
    "I'm dirty dan" => Token::DeclareBool,
    "I went to college" => Token::DeclareString,
    "Where's the leak ma'am?" => Token::SetInitialValue,
    "Are ya ready kids" => Token::BeginMain,
    "Can I be excused for the rest of my life" => Token::EndMain,
    "Wumbo" => Token::Print
};

// assumes a single quote has already been read
// does not currently support escape characters
fn scan_string<I>(it: &mut Peekable<I>) -> Token
where I: Iterator<Item = char>,
{
    let mut str: String = String::new();
    while it.peek().is_some()
    {
        let c: char = it.next().unwrap();
        if c == '\'' {
            break;
        } else {
            str.push(c);
        }
    }
    Token::String(str)
}

fn scan_identifier<I>(it: &mut Peekable<I>, first: char) -> Token
where I: Iterator<Item = char>,
{
    let mut str: String = String::from(first);
    while let Some(c) = it.peek()
    {
        if !(c.is_ascii_alphanumeric() || *c == '_')
        {
            // throw an error here
            break;
        }
        let c: char = it.next().unwrap();
        if c.is_ascii_whitespace() {
            break;
        } else {
            str.push(c);
        }
    }
    Token::Identifier(str)
}

fn scan_number<I>(it: &mut Peekable<I>, first: char) -> Token
where I: Iterator<Item = char>,
{
    let mut str: String = String::from(first);
    while let Some(c) = it.peek()
    {
        if !(c.is_ascii_digit())
        {
            // throw an error here
            break;
        }
        let c: char = it.next().unwrap();
        if c.is_ascii_whitespace() {
            break;
        } else {
            str.push(c);
        }
    }
    Token::Number(str)
}

fn scan_keyword<I>(it: &mut Peekable<I>) -> Token
where I: Iterator<Item = char>,
{
    let mut str: String = String::new();
    while it.peek().is_some()
    {
        let c: char = it.next().unwrap();
        if c == '\'' {
            break;
        } else {
            str.push(c);
        }
    }
    Token::String(str)
}

pub fn tokenize(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut it= source.chars().peekable();
    while let Some(c) = it.next() {
        match c {
            c if c.is_whitespace() => continue,
            // strings
            '\'' => tokens.push(scan_string(&mut it)),
            // identifiers
            c if c.is_ascii_alphabetic() => tokens.push(scan_identifier(&mut it, c)),
            '1'..='9' => tokens.push(scan_number(&mut it, c)),
            // keywords
            '!' => tokens.push(scan_keyword(&mut it)),
            _ => panic!("Unrecognized character")
        }
    }

    tokens
}
