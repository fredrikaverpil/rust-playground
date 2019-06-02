#![allow(dead_code)]
#![allow(unused)]

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
    enumerations();
}