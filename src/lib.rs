//! [TinyExpr](https://github.com/kondrak/codespawn) is a tiny recursive descent expression
//! parser, compiler, and evaluation engine for math expressions.
//! This is a work in progress port of [TinyExpr](https://github.com/codeplea/tinyexpr) to Rust.
//!
//! Current release only supports built-in system functions (trigonometry, algebraic operations, constants, etc.).
//! See the `tests` module for more examples.
//!
//!# Quick Start
//!
//!```
//!extern crate tinyexpr;
//!
//!fn main()
//!{
//!    // parse the expression and fetch result
//!    let r = tinyexpr::interp("2+2*2").unwrap();
//!
//!    // should print "6"
//!    println!("{:?}", r);
//!}
//!```
#[macro_use]
extern crate bitflags;
pub mod error;
use error::Result;
use std::f64::consts;
use std::str::FromStr;

#[doc(hidden)]
bitflags! {
    #[doc(hidden)]
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

// todo: introduce a Function struct to accomodate different arg numbers and ret values?
const FUNCTIONS: [&'static str; 21] = ["abs", "acos", "asin", "atan", "atan2", "ceil", "cos",
                                       "cosh", "e", "exp", "floor", "ln", "log", "log10",
                                       "pi", "pow", "sin", "sinh", "sqrt", "tan", "tanh" ];
const FUNCTION_TYPES: [(fn(f64, f64) -> f64, ExprType); 21] = [ (abs, TE_FUNCTION1), (acos, TE_FUNCTION1), (asin, TE_FUNCTION1),
                                                                 (atan, TE_FUNCTION1), (atan2, TE_FUNCTION2), (ceil, TE_FUNCTION1),
                                                                 (cos, TE_FUNCTION1), (cosh, TE_FUNCTION1), (e, TE_FUNCTION0),
                                                                 (exp, TE_FUNCTION1), (floor, TE_FUNCTION1), (ln, TE_FUNCTION1),
                                                                 (log, TE_FUNCTION1), (log10, TE_FUNCTION1), (pi, TE_FUNCTION0),
                                                                 (pow, TE_FUNCTION2), (sin, TE_FUNCTION1), (sinh, TE_FUNCTION1),
                                                                 (sqrt, TE_FUNCTION1), (tan, TE_FUNCTION1), (tanh, TE_FUNCTION1)];

fn dummy(_: f64, _: f64) -> f64 { panic!("called dummy!") } // todo
fn add(a: f64, b: f64) -> f64 { a + b }
fn sub(a: f64, b: f64) -> f64 { a - b }
fn mul(a: f64, b: f64) -> f64 { a * b }
fn div(a: f64, b: f64) -> f64 { a / b }
fn fmod(a: f64, b: f64) -> f64 { a % b }
fn neg(a: f64, _: f64) -> f64 { -a }
fn comma(_: f64, b: f64) -> f64 { b }
// todo: this is added so that it works with current fptr... - need more types! no extra unused params!
fn abs(a: f64, _: f64) -> f64 { a.abs() }
fn acos(a: f64, _: f64) -> f64 { a.acos() }
fn asin(a: f64, _: f64) -> f64 { a.asin() }
fn atan(a: f64, _: f64) -> f64 { a.atan() }
fn atan2(a: f64, b: f64) -> f64 { a.atan2(b) }
fn ceil(a: f64, _: f64) -> f64 { a.ceil() }
fn cos(a: f64, _: f64) -> f64 { a.cos() }
fn cosh(a: f64, _: f64) -> f64 { a.cosh() }
fn e(_: f64, _: f64) -> f64 { consts::E }
fn exp(a: f64, _: f64) -> f64 { a.exp() }
fn floor(a: f64, _: f64) -> f64 { a.floor() }
fn ln(a: f64, _: f64) -> f64 { a.ln() }
fn log(a: f64, _: f64) -> f64 { a.log10() } // todo ?
fn log10(a: f64, _: f64) -> f64 { a.log10() }
fn pi(_: f64, _: f64) -> f64 { consts::PI }
fn pow(a: f64, b: f64) -> f64 { a.powf(b) }
fn sin(a: f64, _: f64) -> f64 { a.sin() }
fn sinh(a: f64, _: f64) -> f64 { a.sinh() }
fn sqrt(a: f64, _: f64) -> f64 { a.sqrt() }
fn tan(a: f64, _: f64) -> f64 { a.tan() }
fn tanh(a: f64, _: f64) -> f64 { a.tanh() }

#[doc(hidden)]
#[derive(Debug)]
pub struct Expr {
    pub e_type: ExprType,
    pub value: f64,
    pub bound: i8, // todo: Variable?
    pub function: fn(f64, f64) -> f64,
    pub parameters: Vec<Expr> // todo: should this be Option<>? Also, Expr&?
}

impl Expr {
    fn new() -> Expr {
        Expr {
            e_type: TOK_NULL,
            value: 0.0,
            bound: 0,
            function: dummy,
            parameters: Vec::<Expr>::new()
        }
    }
}

impl Clone for Expr {
    fn clone(&self) -> Expr {
        Expr {
            e_type: self.e_type,
            value: self.value,
            bound: self.bound,
            function: self.function,
            parameters: self.parameters.clone()
        }
    }
}


#[doc(hidden)]
#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub address: i8, // todo: this will have to go - handle variables? (no void*)
    pub function: fn(f64, f64) -> f64,
    pub v_type: ExprType,
    pub context: Vec<Expr>,
}

impl Variable {
    fn new(name: &str, v_type: ExprType) -> Variable {
        Variable {
            name: String::from(name),
            address: 0,
            function: dummy,
            v_type: v_type,
            context: Vec::<Expr>::new(),
        }
    }
}

impl Clone for Variable {
    fn clone(&self) -> Variable {
        Variable {
            name: self.name.clone(),
            address: self.address,
            function: self.function,
            v_type: self.v_type,
            context: self.context.clone()
        }
    }
}

#[derive(Debug)]
struct State {
    pub next: String,
    pub s_type: ExprType,
    pub n_idx: usize,
    pub value: f64,
    pub bound: i8,
    pub function: fn(f64, f64) -> f64,
    pub context: Vec<Expr>,
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
            function: mul,
            context: Vec::<Expr>::new(),
            lookup: Vec::<Variable>::new()
        }
    }
}

// todo
fn new_expr(e_type: ExprType, params: Option<Vec<Expr>>) -> Expr {
    let arity = arity!(e_type);
    let mut ret = Expr::new();
    // just create a new expression with new type based on old expression, no weird memcpy mumbo jumbo
    ret.e_type = e_type;
    ret.bound = 0;
    if let Some(params) = params {
        ret.parameters = params;
    }

    ret
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
        let mut v = Variable::new(txt, FUNCTION_TYPES[idx].1 | TE_FLAG_PURE);
        v.function = FUNCTION_TYPES[idx].0;
        return Some(v);
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
                    match type_mask!(v.v_type) {
                        TE_VARIABLE => { s.s_type = TOK_VARIABLE; s.bound = v.address; },
                        TE_CLOSURE0 => s.context = v.context,
                        TE_CLOSURE1 => s.context = v.context,
                        TE_CLOSURE2 => s.context = v.context,
                        TE_CLOSURE3 => s.context = v.context,
                        TE_CLOSURE4 => s.context = v.context,
                        TE_CLOSURE5 => s.context = v.context,
                        TE_CLOSURE6 => s.context = v.context,
                        TE_CLOSURE7 => s.context = v.context,
                        TE_FUNCTION0 => { s.s_type = v.v_type; s.function = v.function; },
                        TE_FUNCTION1 => { s.s_type = v.v_type; s.function = v.function; },
                        TE_FUNCTION2 => { s.s_type = v.v_type; s.function = v.function; },
                        TE_FUNCTION3 => { s.s_type = v.v_type; s.function = v.function; },
                        TE_FUNCTION4 => { s.s_type = v.v_type; s.function = v.function; },
                        TE_FUNCTION5 => { s.s_type = v.v_type; s.function = v.function; },
                        TE_FUNCTION6 => { s.s_type = v.v_type; s.function = v.function; },
                        TE_FUNCTION7 => { s.s_type = v.v_type; s.function = v.function; },
                        _ => {}
                    }
                }
                else {
                    s.s_type = TOK_ERROR;
                }
            } else {
                // look for an operator or special character
                match s.next.as_bytes()[s.n_idx] as char {
                    '+' => { s.s_type = TOK_INFIX; s.function = add; },
                    '-' => { s.s_type = TOK_INFIX; s.function = sub; },
                    '*' => { s.s_type = TOK_INFIX; s.function = mul; },
                    '/' => { s.s_type = TOK_INFIX; s.function = div; },
                    '^' => { s.s_type = TOK_INFIX; s.function = pow; },
                    '%' => { s.s_type = TOK_INFIX; s.function = fmod; },
                    '(' =>  s.s_type = TOK_OPEN,
                    ')' =>  s.s_type = TOK_CLOSE,
                    ',' =>  s.s_type = TOK_SEP,
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
    let mut ret: Expr;

    match type_mask!(s.s_type) {
        TOK_NUMBER => {
            ret = new_expr(TE_CONSTANT, None);
            ret.value = s.value;
            try!(next_token(s));
        },
        TOK_VARIABLE => {
            ret = new_expr(TE_VARIABLE, None);
            ret.bound = s.bound;
            try!(next_token(s));
        },
        TE_FUNCTION0 | TE_CLOSURE0 => {
            ret = new_expr(s.s_type, None);
            ret.function = s.function;
            // todo: set parameters
            /*if is_closure!(s.s_type) {
                ret.parameters[0] = s.context[0].clone();
            }*/
            try!(next_token(s));
            // todo: set parameters
        },
        TE_FUNCTION1 | TE_CLOSURE1 => {
            ret = new_expr(s.s_type, None);
            ret.function = s.function;
            // todo: set parameters
            try!(next_token(s));
            ret.parameters.push(try!(power(s)));
            // todo: set parameters
        },
        TE_FUNCTION2 | TE_CLOSURE2 | TE_FUNCTION3 |
        TE_CLOSURE3 | TE_FUNCTION4 | TE_CLOSURE4 |
        TE_FUNCTION5 | TE_CLOSURE5 | TE_FUNCTION6 |
        TE_CLOSURE6 | TE_FUNCTION7 | TE_CLOSURE7 => {
            let arity = arity!(s.s_type);

            ret = new_expr(s.s_type, None);
            ret.function = s.function;
            // todo: set parameters
            try!(next_token(s));

            if s.s_type != TOK_OPEN {
                s.s_type = TOK_ERROR;
            } else {
                let mut idx = 0;
                for i in 0..arity {
                    try!(next_token(s));
                    ret.parameters.push(try!(expr(s)));
                    if s.s_type != TOK_SEP {
                        break;
                    }
                    idx += 1;
                }
                if s.s_type != TOK_CLOSE || (idx != arity-1) {
                    s.s_type = TOK_ERROR;
                } else {
                    try!(next_token(s));
                }
            }
        },
        TOK_OPEN => {
            try!(next_token(s));
            ret = try!(list(s));
            if s.s_type != TOK_CLOSE {
                s.s_type = TOK_ERROR;
            } else {
                try!(next_token(s));
            }
        }
        _ => {
            // todo: better error? Use NaN?
            ret = new_expr(TE_VARIABLE, None);
            s.s_type = TOK_ERROR;
            ret.value = 0.0;
        }
    }

    Ok(ret)
}

fn power(s: &mut State) -> Result<Expr> {
    let mut sign = 1;

    while s.s_type == TOK_INFIX && (s.function == add || s.function == sub) {
        if s.function == sub { sign = -sign; }
        try!(next_token(s));
    }

    let mut ret: Expr;

    if sign == 1 {
        ret = try!(base(s));
    } else {
        ret = new_expr(TE_FUNCTION1 | TE_FLAG_PURE, Some(vec![try!(base(s)).clone()]));
        ret.function = neg;
    }

    Ok(ret)
}

// todo: ifdef TE_POW_FROM_RIGHT
fn factor(s: &mut State) -> Result<Expr> {
    let mut ret = try!(power(s));

    // todo: check functions here
    while s.s_type == TOK_INFIX && s.function == pow {
        let f = s.function;
        try!(next_token(s));
        ret = new_expr(TE_FUNCTION2 | TE_FLAG_PURE, Some(vec![ret.clone(), try!(power(s)).clone()]));
        ret.function = f;
    }
    
    Ok(ret)
}

fn term(s: &mut State) -> Result<Expr> {
    let mut ret = try!(factor(s));

    while s.s_type == TOK_INFIX && (s.function == mul || s.function == div || s.function == fmod) {
        let f = s.function;
        try!(next_token(s));
        ret = new_expr(TE_FUNCTION2 | TE_FLAG_PURE, Some(vec![ret.clone(), try!(factor(s)).clone()]));
        ret.function = f;
    }
    
    Ok(ret)
}

fn expr(s: &mut State) -> Result<Expr> {
    let mut ret = try!(term(s));

    while s.s_type == TOK_INFIX && (s.function == add || s.function == sub) {
        let f = s.function;
        try!(next_token(s));
        ret = new_expr(TE_FUNCTION2 | TE_FLAG_PURE, Some(vec![ret.clone(), try!(term(s)).clone()]));
        ret.function = f;
    }
    
    Ok(ret)
}

fn list(s: &mut State) -> Result<Expr> {
    let mut ret = try!(expr(s));

    while s.s_type == TOK_SEP {
        try!(next_token(s));
        ret = new_expr(TE_FUNCTION2 | TE_FLAG_PURE, Some(vec![ret.clone(), try!(expr(s)).clone()]));
        ret.function = comma;
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

fn compile(expression: &str, variables: Option<Vec<Variable>> ) -> Result<Option<Expr>> {
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

/// Interprets a string expression as a mathematical expresion, evaluates it and returns its result.
///
/// # Examples
///
/// ```
/// extern crate tinyexpr;
///
/// // "result" should contain a "4"
/// let result = tinyexpr::interp("2+2").unwrap();
/// ```
pub fn interp(expression: &str) -> Result<f64> {
    let e = try!(compile(expression, None));

    if let Some(expr) = e {
        return Ok(eval(&expr));
    }

    Err(error::TinyExprError::Other(String::from("NaN"))) // todo: different error type?
}

// todo
fn eval(n: &Expr) -> f64 {
    match type_mask!(n.e_type) {
        TE_CONSTANT => n.value,
        TE_VARIABLE => n.bound as f64,
        TE_FUNCTION0 | TE_FUNCTION1 | TE_FUNCTION2 | TE_FUNCTION3 |
        TE_FUNCTION4 | TE_FUNCTION5 | TE_FUNCTION6 | TE_FUNCTION7 => {
            match arity!(n.e_type) {
                // todo: REALLY need more function pointer types to avoid hacks like this 0.0 here...
                0 => ((*n).function)(0.0, 0.0),
                1 => ((*n).function)(eval(&n.parameters[0]), 0.0),
                2 => ((*n).function)(eval(&n.parameters[0]), eval(&n.parameters[1])),
                _ => panic!("todo: add more f. pointers (type is {})", arity!(n.e_type))
            }
        }
        _ => 0.0
    }
}
