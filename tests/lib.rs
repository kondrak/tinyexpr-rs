extern crate tinyexpr;

#[test]
fn foo() {
    tinyexpr::interp();
    tinyexpr::compile();
    tinyexpr::eval();
}
