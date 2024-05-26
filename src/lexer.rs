use std::iter::Peekable;
use std::str::Chars;

const operators: [&str; 11] = ["||", "&&", "==", "=", "+", "-", "*", "/", ">", "<", "!" ];
const datatypes: [&str; 3]  = ["str", "int", "bool"];
const keywords : [&str; 8]  = ["else", "elif", "if", "fi", "true", "false", "out", "read"];

pub fn lex(code: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = code.chars().peekable();

    while let Some(&ch) = chars.peek() {

        if ch == '"' {
            tokens.push(Token::Literal(LitValue::String(get_string(&mut chars))));
            continue;
        }
        // if let Some(symbol) = get_symbol(&mut chars) {
        //     tokens.push(Token::Symbol(symbol));
        //     continue;
        // }

        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        if let Some(op) = get_operator(&mut chars) {
            tokens.push(Token::Op(op));
            continue;
        }

        if let Some(keyword) = get_keyword(&mut chars) {
            if keyword == "true"{
                tokens.push(Token::Literal(LitValue::Bool(true)));
            }
            else if keyword == "false"{
                tokens.push(Token::Literal(LitValue::Bool(false)));
            }
            else {
                tokens.push(Token::Keyword(keyword));
            }
            continue;
        }

        if let Some(dt) = get_datatype(&mut chars) {
            tokens.push(Token::DTypeToken(dt));
            continue;
        }

        if ch.is_alphabetic() || ch == '_' {
            let identifier = get_identifier(&mut chars);
            tokens.push(Token::Id(identifier));
            continue;
        }

        if ch.is_numeric() {
            let number = get_number(&mut chars);
            tokens.push(Token::Literal(LitValue::Int(number)));
            continue;
        }

        

        panic!("Unknown symbol: {}", ch);
    }

    tokens
}

fn get_remaining_chars_as_str(chars: &Peekable<Chars>) -> String {
    chars.clone().collect::<String>()
}

fn get_operator(chars: &mut Peekable<Chars>) -> Option<String> {
    let remaining_chars = get_remaining_chars_as_str(chars);

    for &op in &operators {
        
        if remaining_chars.starts_with(op){
            for _ in 0..op.len() {
                chars.next();
            }
            return Some(op.to_string());
        }
    }
    None
}

// fn get_symbol(chars: &mut Peekable<Chars>) -> Option<String> {
//     let symbols = ["\n"];
//     if let Some(&ch) = chars.peek() {
//         if symbols.contains(&ch.to_string().as_str()) {
//             chars.next();
//             return Some(ch.to_string());
//         }
//     }
//     None
// }

fn get_keyword(chars: &mut Peekable<Chars>) -> Option<String> {
    let remaining_chars = get_remaining_chars_as_str(chars);

    for &kw in &keywords {
        if remaining_chars.starts_with(kw) && (kw.len() == remaining_chars.len() || remaining_chars.chars().nth(kw.len()).unwrap().is_whitespace()) {
            for _ in 0..kw.len() {
                chars.next();
            }
            return Some(kw.to_string());
        }
    }
    None
}

fn get_datatype(chars: &mut Peekable<Chars>) -> Option<DataType> {
    let remaining_chars = get_remaining_chars_as_str(chars);

    for &dt in &datatypes {
        if remaining_chars.starts_with(dt) {
            for _ in 0..dt.len() {
                chars.next();
            }
            if dt.to_string() == "int"{
                return Some(DataType::Integer);
            }
            
        }
    }
    None
}

fn get_identifier(chars: &mut Peekable<Chars>) -> String {
    let mut identifier = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            identifier.push(ch);
            chars.next();
        } else {
            break;
        }
    }
    identifier
}

fn get_number(chars: &mut Peekable<Chars>) -> i64 {
    let mut num_str = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_numeric() {
            num_str.push(ch);
            chars.next();
        } else {
            break;
        }
    }
    num_str.parse().expect("Invalid number")
}

fn get_string(chars: &mut Peekable<Chars>) -> String {
    chars.next();
    let mut str_v = String::new();
    while let Some(&ch) = chars.peek() {
        if ch != '"' {
            str_v.push(ch);
            chars.next();
        } else {
            chars.next();
            break;
        }
    }
    str_v
}

#[derive(Debug)]
#[derive(Clone)]
pub enum DataType {
    Integer,
    Boolean,
    String,
    // Add other variants as needed
}

#[derive(Debug)]
#[derive(Clone)]
pub enum LitValue {
    Int(i64),
    Bool(bool),
    String(String)
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Id(String),
    Symbol(String),
    Op(String),
    Literal(LitValue),
    DTypeToken(DataType)
}
