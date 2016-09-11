extern crate tinyexpr;

fn main()
{
    let r = tinyexpr::interp("2*1/sin(3.14/2)").unwrap_or_else(|e| {
        panic!("{}", e);
    });

    println!("{:?}", r);
}
