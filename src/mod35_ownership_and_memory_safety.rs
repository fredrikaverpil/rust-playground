#![allow(unused)]

fn ownership1() {
    let v = vec![1, 2, 3]; // v happens to own what it is bound to, owns the memory stored in the vector
                           // v is stored on the stack but its value/data is stored on the heap
                           // only one variable binds to a resource!

    let v2 = v; // copy the pointer, v becomes unusable

    // println!("{:?}", v);  // will not compile!
}

fn ownership2() {
    let v = vec![1, 2, 3];

    let foo = |x: Vec<i32>| ();
    foo(v); // renders v unusable

    // println!("{:?}", v);  // will not compile!
}

fn ownership3() {
    let u = 1;
    let u2 = u;
    println!("u = {}", u); // primitives like i32 are okay, a full copy is performed and it is rather cheap to do

    let v = Box::new(1); // place v on stack, 1 on heap
    let v2 = v; // renders v unusable
                // println!("{:?}", *v);  // will not compile!
}

fn ownership4() {
    // one solution to deal with vector lambda example;
    // let the function return the vector
    let v = vec![1, 2, 3];

    let foo = |x: Vec<i32>| -> Vec<i32> {
        println!("foo: {:?}", x);
        x
    };

    let v = foo(v);
    println!("{:?}", v);
}

pub fn main() {
    ownership1();
    ownership2();
    ownership3();
    ownership4();
}
