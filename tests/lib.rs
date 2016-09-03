extern crate tinyexpr;

#[test]
fn foo() {
    tinyexpr::interp("2*2");
    tinyexpr::compile();
    tinyexpr::eval();
}
