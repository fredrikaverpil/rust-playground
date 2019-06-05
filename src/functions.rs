
fn print_value(x: i32) {
    println!("{}", x);
}

fn increase(x: &mut i32) {
    *x += 1;  // de-reference, increase and return
}


fn product(x: i32, y: i32) -> i32 {
    x * y  // return statement does not end with semicolon ;
}

fn functions() {
    print_value(33);

    let mut z = 1;
    increase(&mut z);  // pass as mutable reference
    println!("z = {}", z);

    let a = 3;
    let b = 4;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);
}

pub fn main() {
    functions()
}