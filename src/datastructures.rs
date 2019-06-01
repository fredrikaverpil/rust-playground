#![allow(dead_code)]
#![allow(unused)]

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p1 = Point {x: 0.0, y: 0.0};
    println!("point p1 is at ({}, {})", p1.x, p1.y);

    let p2 = Point {x: 3.0, y: 10.0};
    println!("point p2 is at ({}, {})", p2.x, p2.y);

    let line = Line {start: p1, end: p2};
}


enum Color {
    Red,
    Green,
    Blue,
    RgbColor (u8, u8, u8),  // tuple like construct
    CmykColor {cyan: u8, magenta: u8, yellow: u8, black: u8}  // struct
}

fn enumerations() {
    // let c:Color = Color::Red;
    // let c:Color = Color::RgbColor(0, 0, 0);
    let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 0};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor{cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
        Color::RgbColor (r, g, b) => println!("rgb {},{},{}", r, g, b),
        Color::CmykColor {cyan, magenta, yellow, black} => println!("cmyk {},{},{},{}", cyan, magenta, yellow, black)
    }
}

pub fn main() {
    structures();
    enumerations();
}