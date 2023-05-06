#![allow(dead_code)]
#![allow(unused)]

// Note, structs and their attributes are made public ("pub") because used elsewhere too

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

fn structures() {
    let p1 = Point { x: 0.0, y: 0.0 };
    println!("point p1 is at ({}, {})", p1.x, p1.y);

    let p2 = Point { x: 3.0, y: 10.0 };
    println!("point p2 is at ({}, {})", p2.x, p2.y);

    let line = Line { start: p1, end: p2 };
}

pub fn main() {
    structures();
}
