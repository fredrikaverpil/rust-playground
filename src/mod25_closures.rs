#![allow(unused)]

fn say_hello() {
    println!("hello");
}

fn closures() {
    let sh = say_hello; // it is possible to create a function and store it in a variable
    sh();

    // lambda-like function which specifies the argument type of x as i32 and return type as i32
    // followed by the body of the function
    let plus_one = |x: i32| -> i32 { x + 1 }; // this function is available in closures function only
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    let plus_two = |x|  // let the compiler figure out the types
    {
        let mut z = x;
        z += 2;
        // z += two;  // closure borrows 'two' until function scope is complete and 'two' is released
        z
    };
    println!("{} + 2 = {}", 3, plus_two(3));
    // let borrow_two = &mut two;  // this is not allowed

    // to fix borrow situation issue above, add a scope
    {
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        // scope with borrowed 'two' is destroyed here and 'two' is released
        println!("{} + 2 = {}", 3, plus_two(3));
    }
    let borrow_two = &mut two; // this is not allowed

    // T:       - by value
    // T&       - reference
    // &mut &   - mutable reference
    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {} ", f);
}

pub fn main() {
    closures()
}
