pub mod error;
use error::Result;
use std::str::FromStr;

enum ExprType {
    Variable,
    Function0,
    Function1,
    Function2,
    Function3,
    Function4,
    Function5,
    Function6,
    Function7,
    Closure0,
    Closure1,
    Closure2,
    Closure3,
    Closure4,
    Closure5,
    Closure6,
    Closure7,
    FlagPure
}

#[derive(PartialEq)]
enum TokenType {
    Null,
    Error,
    End,
    Sep,
    Open,
    Close,
    Number,
    Variable,
    Infix
}

struct State {
    pub start: String,
    pub next: String,
    pub s_type: TokenType,
    pub value: f64,
}

fn find_lookup(s: &State, txt: &String) -> u8 {
    0
}

fn find_builtin(txt: &String) -> u8 {
    0
}

fn next_token(s: &mut State) -> Result<String> {
    s.s_type = TokenType::Null;

    let mut idx: usize = 0;
    
    while s.s_type == TokenType::Null {
        if idx == s.next.len() {
            s.s_type = TokenType::End;
            break;
        }

        let next_char = s.next.as_bytes()[idx] as char;
        // try reading a number
        if (next_char >= '0' && next_char <= '9') || next_char == '.' {
            let mut num_str = String::new();
            let mut c = next_char;

            // extract the number part to separate string which we then convert to f64
            while idx < s.next.len() && (next_char >= '0' && next_char <= '9') || c == '.' {
                c = s.next.as_bytes()[idx] as char;
                num_str.push(c);
                idx += 1;
            }
            s.value  = try!(f64::from_str(&num_str));
            s.s_type = TokenType::Number;
        } else {
            // look for a variable or builting function call
            if next_char >= 'a' && next_char <= 'z' {
                let mut txt_str = String::new();
                let mut c = next_char;

                while idx < s.next.len() && (next_char >= 'a' && next_char <= 'z') || (next_char >= '0' && next_char <= '9') {
                    c = s.next.as_bytes()[idx] as char;
                    txt_str.push(c);
                    idx += 1;
                }

                let mut var = find_lookup(&s, &txt_str);
                if var == 0 {
                   var = find_builtin(&txt_str);
                }

                if var == 0 {
                    s.s_type = TokenType::Error;
                } else {
                    // todo
                }
            } else {
                // look for an operator or special character
                match s.next.as_bytes()[idx] as char {
                    '+' => s.s_type = TokenType::Infix,
                    '-' => s.s_type = TokenType::Infix,
                    '*' => s.s_type = TokenType::Infix,
                    '/' => s.s_type = TokenType::Infix,
                    '^' => s.s_type = TokenType::Infix,
                    '%' => s.s_type = TokenType::Infix,
                    '(' => s.s_type = TokenType::Open,
                    ')' => s.s_type = TokenType::Close,
                    ',' => s.s_type = TokenType::Sep,
                    ' ' | '\t' | '\n' |'\r' => {},
                      _ => s.s_type = TokenType::Error
                }
                idx += 1;
            }
        }
    }

    Ok(String::new())
}

pub fn interp(expression: &str) -> Result<String> {
    Ok(String::new())
}

pub fn compile() {
}

pub fn eval() {
}
