extern crate tinyexpr;

fn main()
{
    let r = tinyexpr::interp("2*2+1/2").unwrap_or_else(|e| {
        panic!("{}", e);
    });

    println!("{:?}", r);
}
