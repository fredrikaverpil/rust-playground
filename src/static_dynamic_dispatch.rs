#![allow(unused)]

use std::mem;

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

// Static dispatch, compilation-time, efficient call
fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
}  // monomorphisation (as it supports both i32 and String)

// Dynamic dispatch, runtime, expensive call
fn print_it_dynamic(z: &dyn Printable) {
    println!("{}", z.format());
}

fn static_dispatch() {
    let a = 123;
    let b = "hello".to_string();

    // println!("{}", a.format());
    // println!("{}", b.format());

    print_it(a);
    print_it(b);
}

fn dynamic_dispatch() {
    let a = 123;
    let b = "hello".to_string();

    // println!("{}", a.format());
    // println!("{}", b.format());

    print_it_dynamic(&a);
    print_it_dynamic(&b);
}

struct Circle { radius: f64 }
struct Square { side: f64 }

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn dynamic_dispatch_only() {
    let shapes: [&dyn Shape; 4] = [
        &Circle{radius: 1.0},
        &Square{side: 3.0},
        &Circle{radius: 2.0},
        &Square{side: 4.0}
    ];
    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape #{} has area {}", i, shape.area());  // dynamic dispatch
    }
}


pub fn main() {
    // solve the same problem using either;
    static_dispatch();
    dynamic_dispatch();

    // problem that can only be solved with dynamic dispatch
    dynamic_dispatch_only();
}