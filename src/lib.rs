#[macro_use]
extern crate bitflags;
pub mod error;
use error::Result;
use std::str::FromStr;

bitflags! {
    flags ExprType: u64 {
        const TE_VARIABLE = 0,
        const TE_FUNCTION0 = 8,
        const TE_FUNCTION1 = 9,
        const TE_FUNCTION2 = 10,
        const TE_FUNCTION3 = 11,
        const TE_FUNCTION4 = 12,
        const TE_FUNCTION5 = 13,
        const TE_FUNCTION6 = 14,
        const TE_FUNCTION7 = 15,
        const TE_CLOSURE0 = 16,
        const TE_CLOSURE1 = 17,
        const TE_CLOSURE2 = 18,
        const TE_CLOSURE3 = 19,
        const TE_CLOSURE4 = 20,
        const TE_CLOSURE5 = 21,
        const TE_CLOSURE6 = 22,
        const TE_CLOSURE7 = 23,
        const TE_FLAG_PURE = 32,
        const TOK_NULL = 24,
        const TOK_ERROR = 25,
        const TOK_END = 26,
        const TOK_SEP = 27,
        const TOK_OPEN = 28,
        const TOK_CLOSE = 29,
        const TOK_NUMBER = 30,
        const TOK_VARIABLE = 31,
        const TOK_INFIX = 32,
        const T_MASK = 0x0000001F
    }
}

struct Variable {
    pub name: String,
    pub address: i8,
    pub v_type: ExprType,
    pub context: i8,
}

impl Variable {
    fn new() -> Variable {
        Variable {
            name: String::new(),
            address: 0,
            v_type: TE_VARIABLE,
            context: 0
        }
    }
}

struct State {
    pub next: String,
    pub s_type: ExprType,
    pub n_idx: usize,
    pub value: f64,
    pub bound: i8,
    pub function: i8,
    pub context: i8
}

impl State {
    fn new() -> State {
        State {
            next: String::new(),
            s_type: TOK_NULL,
            n_idx: 0,
            value: 0.0,
            bound: 0,
            function: 0,
            context: 0
        }
    }
}

fn find_lookup(s: &State, txt: &String) -> Option<Variable> {
    Some(Variable::new())
}

fn find_builtin(s: &State, txt: &String) -> Option<Variable> {
    Some(Variable::new())
}

fn next_token(s: &mut State) -> Result<String> {
    s.s_type = TOK_NULL;
    
    while s.s_type == TOK_NULL {
        if s.n_idx == s.next.len() {
            s.s_type = TOK_END;
            break;
        }

        let next_char = s.next.as_bytes()[s.n_idx] as char;
        // try reading a number
        if (next_char >= '0' && next_char <= '9') || next_char == '.' {
            let mut num_str = String::new();
            let mut c = next_char;

            // extract the number part to separate string which we then convert to f64
            while s.n_idx < s.next.len() && (c >= '0' && c <= '9') || c == '.' {
                num_str.push(c);
                s.n_idx += 1;
                if s.n_idx < s.next.len() {
                    c = s.next.as_bytes()[s.n_idx] as char;
                }
            }
            s.value  = try!(f64::from_str(&num_str));
            s.s_type = TOK_NUMBER;
        } else {
            // look for a variable or builting function call
            if next_char >= 'a' && next_char <= 'z' {
                let mut txt_str = String::new();
                let mut c = next_char;

                while s.n_idx < s.next.len() && (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') {
                    txt_str.push(c);
                    s.n_idx += 1;
                    if s.n_idx < s.next.len() {
                        c = s.next.as_bytes()[s.n_idx] as char;
                    }
                }

                let mut var = find_lookup(&s, &txt_str);
                if let None = var {
                   var = find_builtin(&s, &txt_str);
                }

                if let Some(v) = var {
                    match s.s_type & T_MASK {
                        TE_VARIABLE => { s.s_type = TOK_VARIABLE; s.bound = v.address; },
                        TE_CLOSURE0 => s.context = v.context,
                        TE_CLOSURE1 => s.context = v.context,
                        TE_CLOSURE2 => s.context = v.context,
                        TE_CLOSURE3 => s.context = v.context,
                        TE_CLOSURE4 => s.context = v.context,
                        TE_CLOSURE5 => s.context = v.context,
                        TE_CLOSURE6 => s.context = v.context,
                        TE_CLOSURE7 => s.context = v.context,
                        TE_FUNCTION0 => { s.s_type = v.v_type; s.function = v.address; },
                        TE_FUNCTION1 => { s.s_type = v.v_type; s.function = v.address; },
                        TE_FUNCTION2 => { s.s_type = v.v_type; s.function = v.address; },
                        TE_FUNCTION3 => { s.s_type = v.v_type; s.function = v.address; },
                        TE_FUNCTION4 => { s.s_type = v.v_type; s.function = v.address; },
                        TE_FUNCTION5 => { s.s_type = v.v_type; s.function = v.address; },
                        TE_FUNCTION6 => { s.s_type = v.v_type; s.function = v.address; },
                        TE_FUNCTION7 => { s.s_type = v.v_type; s.function = v.address; },
                        _ => {}
                    }
                }
                else {
                    s.s_type = TOK_ERROR;
                }
            } else {
                // todo: set function
                // look for an operator or special character
                match s.next.as_bytes()[s.n_idx] as char {
                    '+' => s.s_type = TOK_INFIX,
                    '-' => s.s_type = TOK_INFIX,
                    '*' => s.s_type = TOK_INFIX,
                    '/' => s.s_type = TOK_INFIX,
                    '^' => s.s_type = TOK_INFIX,
                    '%' => s.s_type = TOK_INFIX,
                    '(' => s.s_type = TOK_OPEN,
                    ')' => s.s_type = TOK_CLOSE,
                    ',' => s.s_type = TOK_SEP,
                    ' ' | '\t' | '\n' |'\r' => {},
                      _ => s.s_type = TOK_ERROR
                }
                s.n_idx += 1;
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
