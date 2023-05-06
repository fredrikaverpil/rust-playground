#![allow(dead_code)] // Point.y is never used. Allow this.

use std::mem;

struct Point {
    // 64 bits = 8 bytes
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn main() {
    let p1 = origin(); // allocate on stack (short-term)
    let p2 = Box::new(origin()); // allocate on heap (long-term)

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let _p3 = *p2; // reallocate p2 back onto the stack by following the pointer (dereference)

    println!("{}", _p3.x);
}
