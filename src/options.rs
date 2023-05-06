fn option() {
    // Option<T>

    let x = 3.0;
    let y = 2.0;

    // Option can contain Some(z) or None
    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };

    println!("{:?}", result);

    // How can you destructure and get the result of the value?
    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y),
    }

    // if let / while let
    if let Some(z) = result {
        println!("z = {}", z);
    }
}

pub fn main() {
    option();
}
