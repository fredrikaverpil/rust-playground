#[allow(unused_variables)]

// like list in python

fn vectors() {
    // Think of vectors as a stack where you push values on to the top
    // Or a list where you append to the head

    let mut a = Vec::new();

    a.push(1); // add value
    a.push(2); // add value
    a.push(3); // add value

    println!("{:?}", a);

    a.push(4);

    println!("{:?}", a);

    println!("a[0] = {}", a[0]);

    // What is the type of the index?
    // usize isize
    let idx: i32 = 0;
    // println!("a[0] = {}", a[idx]);  // will not work, cannot use signed value

    let idx: usize = 0;
    println!("a[0] = {}", a[idx]); // ok!

    // a[6]; // panic
    let option_slice = a.get(6); // returns Option

    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element"),
    }

    for x in &a {
        println!("{}", x);
    }

    a.push(77);
    println!("{:?}", a);

    let last_element = a.pop(); // Returns an Option, in this case Some(77)
                                // Rust does not crash if you try to pop an empty vector
    println!("last element is {:?}, a = {:?}", last_element, a);

    a.insert(0, 987); // insert at index 0
    println!("{:?}", a);

    // reverse
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

pub fn main() {
    vectors()
}
