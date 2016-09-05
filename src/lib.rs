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

fn find_lookup(s: &State, name: char) -> u8 {
    0
}

fn find_builtin(name: char) -> u8 {
    0
}

fn next_token(s: &mut State) -> Result<String> {
    s.s_type = TokenType::Null;

    let mut idx: usize = 0;
    
    while s.s_type == TokenType::Null {
        if s.s_type == TokenType::End {
            break;
        }

        let first_byte = s.next.as_bytes()[0];
        // try reading a number
        // 0-9 or .
        if (first_byte >= 48 && first_byte < 57) || first_byte == 46 {
            s.value  = try!(f64::from_str(&s.next));
            s.s_type = TokenType::Number;
        } else {
            // look for a variable or builting function call
            // a-z
            if first_byte >= 97 && first_byte <= 122 {
                let mut start = s.next.as_bytes()[idx];
                let mut c = start;
                while (c >= 97 && c <= 122) || (c >= 48 && c <= 57) {
                    idx += 1;
                    c = s.next.as_bytes()[idx];
                }

                let mut var = find_lookup(&s, start as char);
                if var == 0 {
                   var = find_builtin(start as char);
                }

                if var == 0 {
                    s.s_type = TokenType::Error;
                } else {
                    // todo
                }
            } else {
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
                // look for an operator or special character
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
