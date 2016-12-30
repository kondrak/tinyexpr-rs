extern crate tinyexpr;

fn main()
{
    // evaluate expression and fetch result
    let result = tinyexpr::interp("2*1/sin(pi/2)").unwrap_or_else(|e| {
        panic!("{}", e);
    });

    println!("{:?}", result);
}
