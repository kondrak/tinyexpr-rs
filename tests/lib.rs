extern crate tinyexpr;

#[test]
fn foo() {
    assert_eq!(tinyexpr::interp("2*2").unwrap(), 4.0);
}
