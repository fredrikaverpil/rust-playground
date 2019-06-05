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
fn print_it_dynamic(z: &Printable) {
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

pub fn main() {
    static_dispatch();
    dynamic_dispatch();
}