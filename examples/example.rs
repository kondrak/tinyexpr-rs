extern crate tinyexpr;

fn main()
{
    // Example of parsing a simplified JSON definition.
    // 'name' and 'type' can be omitted and will be automatically deduced by the parser.
    let _ = tinyexpr::interp("2*2").unwrap_or_else(|e| {
        panic!("{}", e);
    });
}
