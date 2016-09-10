extern crate tinyexpr;

fn main()
{
    let _ = tinyexpr::compile("2*2", None).unwrap_or_else(|e| {
        panic!("{}", e);
    });
}
