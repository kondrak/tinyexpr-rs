extern crate tinyexpr;

#[test]
fn check_basics() {
    assert_eq!(tinyexpr::interp("2*2").unwrap(), 4.0);
    assert_eq!(tinyexpr::interp("2+2").unwrap(), 4.0);
    assert_eq!(tinyexpr::interp("3-2").unwrap(), 1.0);
    assert_eq!(tinyexpr::interp("5%2").unwrap(), 1.0);
    assert_eq!(tinyexpr::interp("5^2").unwrap(), 25.0);
    assert_eq!(tinyexpr::interp("2+2*2").unwrap(), 6.0);
    assert_eq!(tinyexpr::interp("(2+2)*2").unwrap(), 8.0);
    assert_eq!(tinyexpr::interp("(2+2)*2/2").unwrap(), 4.0);
    assert_eq!(tinyexpr::interp("abs(-1)").unwrap(), 1.0);
    assert_eq!(tinyexpr::interp("sqrt(728*728)").unwrap(), 728.0);
    assert_eq!(tinyexpr::interp("pow(2.0, 3.0)").unwrap(), 8.0);
    assert_eq!(tinyexpr::interp("exp(1)").unwrap(), tinyexpr::interp("e").unwrap());
    assert_eq!(tinyexpr::interp("floor(3.1415)").unwrap(), 3.0);
    assert_eq!(tinyexpr::interp("ceil(3.1415)*floor(3.1415)").unwrap(), 12.0);
    assert_eq!(tinyexpr::interp("5,2").unwrap(), 2.0);
}

#[test]
fn check_constants() {
    assert_eq!(tinyexpr::interp("pi").unwrap(), 3.141592653589793);
    assert_eq!(tinyexpr::interp("e").unwrap(), 2.718281828459045);
}

#[test]
fn check_logarithms() {
    assert_eq!(tinyexpr::interp("ln(e)").unwrap(), 1.0);
    assert_eq!(tinyexpr::interp("log(10)").unwrap(), 1.0);
    assert_eq!(tinyexpr::interp("log10(10)").unwrap(), 1.0);
}


#[test]
fn check_trigs() {
    assert_eq!(tinyexpr::interp("2*1/sin(3.14/2)").unwrap().round(), 2.0);
    assert_eq!(tinyexpr::interp("asin(1)").unwrap(), tinyexpr::interp("pi/2").unwrap());
    assert_eq!(tinyexpr::interp("tan(pi)").unwrap().round(), 0.0);
    assert_eq!(tinyexpr::interp("atan(pi/2)").unwrap().round(), 1.0);
    assert_eq!(tinyexpr::interp("atan2(pi, 2)").unwrap().round(), 1.0);
    assert_eq!(tinyexpr::interp("cos(0)").unwrap().round(), 1.0);
    assert_eq!(tinyexpr::interp("acos(1)").unwrap(), 0.0);
}

#[test]
fn check_hyberbolic_trigs() {
    assert_eq!(tinyexpr::interp("sinh(0)").unwrap(), 0.0);
    assert_eq!(tinyexpr::interp("cosh(0)").unwrap(), 1.0);
    assert_eq!(tinyexpr::interp("tanh(10000)").unwrap(), 1.0);
}

#[test]
#[should_panic]
fn parse_error()
{ let _ = tinyexpr::interp("atan(foo)").unwrap_or_else(|e| { panic!("{}", e); }); }
