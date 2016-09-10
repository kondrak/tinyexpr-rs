#[macro_use]
extern crate bitflags;
pub mod error;
use error::Result;
use std::str::FromStr;

bitflags! {
    pub flags ExprType: u64 {
        const TE_VARIABLE = 0,
        const TE_CONSTANT = 1,
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
        const T_MASK =  0x0000001F
    }
}

macro_rules! type_mask {
    ($x:expr) => ($x & T_MASK)
}

macro_rules! is_pure {
    ($x:expr) => (($x & TE_FLAG_PURE).bits() != 0)
}

macro_rules! is_function {
    ($x:expr) => (($x & TE_FUNCTION0).bits() != 0)
}

macro_rules! is_closure {
    ($x:expr) => (($x & TE_CLOSURE0).bits() != 0)
}

macro_rules! arity {
    ($x:expr) => (if($x & (TE_FUNCTION0 | TE_CLOSURE0)).bits() != 0 { $x.bits() & 0x00000007 } else { 0 })
}

// todo: function pointers?
const FUNCTIONS: [&'static str; 21] = ["abs", "acos", "asin", "atan", "atan2", "ceil", "cos",
                                       "cosh", "e", "exp", "floor", "ln", "log", "log10",
                                       "pi", "pow", "sin", "sinh", "sqrt", "tan", "tanh" ];
const FUNCTION_TYPES: [ExprType; 21] = [ TE_FUNCTION1, TE_FUNCTION1, TE_FUNCTION1, TE_FUNCTION1,
                                         TE_FUNCTION2, TE_FUNCTION1, TE_FUNCTION1, TE_FUNCTION1,
                                         TE_FUNCTION0, TE_FUNCTION1, TE_FUNCTION1, TE_FUNCTION1,
                                         TE_FUNCTION1, TE_FUNCTION1, TE_FUNCTION1, TE_FUNCTION0,
                                         TE_FUNCTION2, TE_FUNCTION1, TE_FUNCTION1, TE_FUNCTION1,
                                         TE_FUNCTION1];
pub struct Expr {
    pub e_type: ExprType,
    pub value: f64,
    pub bound: i8,
    pub function: i8,
    pub parameters: i8
}

impl Expr {
    fn new() -> Expr {
        Expr {
            e_type: TOK_NULL,
            value: 0.0,
            bound: 0,
            function: 0,
            parameters: 0,
        }
    }
}

pub struct Variable {
    pub name: String,
    pub address: i8,
    pub v_type: ExprType,
    pub context: i8,
}

impl Variable {
    fn new(name: &str, v_type: ExprType) -> Variable {
        Variable {
            name: String::from(name),
            address: 0,
            v_type: v_type,
            context: 0,
        }
    }
}

impl Clone for Variable {
    fn clone(&self) -> Variable {
        Variable {
            name: self.name.clone(),
            address: self.address,
            v_type: self.v_type,
            context: self.context
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
    pub context: i8,
    pub lookup: Vec<Variable>,
}

impl State {
    fn new(expression: &str) -> State {
        State {
            next: String::from(expression),
            s_type: TOK_NULL,
            n_idx: 0,
            value: 0.0,
            bound: 0,
            function: 0,
            context: 0,
            lookup: Vec::<Variable>::new()
        }
    }
}

fn find_lookup(s: &State, txt: &str) -> Option<Variable> {
    for var in &s.lookup {
        if &(*var.name) == txt {
            return Some((*var).clone());
        }
    }
    
    None
}

fn find_builtin(txt: &str) -> Option<Variable> {
    if let Ok(idx) = FUNCTIONS.binary_search(&txt) {
        return Some(Variable::new(txt, FUNCTION_TYPES[idx]));
    }
    None
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
                   var = find_builtin(&txt_str);
                }

                if let Some(v) = var {
                    match type_mask!(s.s_type) {
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

fn base(s: &mut State) -> Result<Expr> {
    Ok(Expr::new())
}

fn power(s: &mut State) -> Result<Expr> {
    let sign = 1;
    // todo: check functions here
    while s.s_type == TOK_INFIX {
        try!(next_token(s));
    }

    // todo: new_expr if sign != 1, set function
    let ret = try!(base(s));

    Ok(ret)
}

// todo: ifdef TE_POW_FROM_RIGHT
fn factor(s: &mut State) -> Result<Expr> {
    let ret = try!(power(s));

    // todo: check functions here
    while s.s_type == TOK_INFIX {
        // todo: fetch and set functions
        try!(next_token(s));
    }
    
    Ok(ret)
}

fn term(s: &mut State) -> Result<Expr> {
    let ret = try!(factor(s));

    // todo: check functions here
    while s.s_type == TOK_INFIX {
        // todo: fetch and set functions
        try!(next_token(s));
    }
    
    Ok(ret)
}

fn expr(s: &mut State) -> Result<Expr> {
    let ret = try!(term(s));

    // todo: check functions here
    while s.s_type == TOK_INFIX {
        // todo: fetch and set functions
        try!(next_token(s));
    }
    
    Ok(ret)
}

fn list(s: &mut State) -> Result<Expr> {
    let ret = try!(expr(s));

    while s.s_type == TOK_SEP {
        try!(next_token(s));
        // todo: new expr
        // todo: set function
    }
    
    Ok(ret)
}

fn optimize(n: &mut Expr) {
    // evaluates as much as possible
    if n.e_type == TE_CONSTANT { return; }
    if n.e_type == TE_VARIABLE { return; }

    if (n.e_type & TE_FLAG_PURE).bits() != 0  {
        let mut known = 1;
        let arity = arity!(n.e_type);
        
        for i in 0..arity {
            // todo: optimize parameters
        }

        if known != 0 {
            n.value = eval(&n);
            n.e_type = TE_CONSTANT;
        }
    }
}

pub fn compile(expression: &str, variables: Option<Vec<Variable>> ) -> Result<Option<Expr>> {
    let mut s = State::new(expression);
    if let Some(vars) = variables {
        s.lookup = vars;
    }

    arity!(s.s_type);
    try!(next_token(&mut s));
    let mut root = try!(list(&mut s));

    if s.s_type != TOK_END {
        return Ok(None)
    }

    optimize(&mut root);
    Ok(Some(root))
}

pub fn interp(expression: &str) -> Result<f64> {
    let e = try!(compile(expression, None));

    if let Some(expr) = e {
        return Ok(eval(&expr));
    }

    Err(error::TinyExprError::Other(String::from("NaN")))
}

pub fn eval(n: &Expr) -> f64 {
    0.0
}
