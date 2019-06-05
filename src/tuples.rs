#![allow(unused)]

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructure
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    // tuples in tuples
    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);
    let ((c, d), (e, f)) = combined; // destructure

    // mixed
    let foo = (true, "hello", 42, -1.2);
    println!("{:?}", foo);

    // one-element tuple
    let meaning = (42,);  // cannot skip the trailing comma
    println!("{:?}", meaning);

}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x+y, x*y)
}

pub fn main() {
    tuples()
}