#![allow(unused)]

use crate::enums;

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        z @ 9..=11 => "lots of",
        _ if (x % 2 == 0) => "some",
        _ => "a few"
    }
}

fn where_is(point: (i32, i32)) {
    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (ref x, y) => println!("({}, {})", x, y),  // reference syntax example
        (_, y) => println!("(?, {})", y)  // example syntax, this case will never be hit
    }
}

fn which_color(c: enums::Color) {
    match c {
        enums::Color::Red => println!("r"),
        enums::Color::Green => println!("g"),
        enums::Color::Blue => println!("b"),
        enums::Color::RgbColor(0, 0, 0)
        | enums::Color::CmykColor{black: 255,..} => println!("black"),  // the .. indicates we are only interested in the black value
        enums::Color::RgbColor (r, g, b) => println!("rgb {},{},{}", r, g, b),
        enums::Color::CmykColor {cyan, magenta, yellow, black} => println!("cmyk {},{},{},{}", cyan, magenta, yellow, black)
    }
}

fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (3, 4);
    where_is(point);
    where_is((0, 7));
    where_is((1, 0));
    where_is((5, 6));

    let mut c: enums::Color = enums::Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 0};
    which_color(c);
    c = enums::Color::RgbColor(150, 150, 150);
    which_color(c);
    c = enums::Color::RgbColor(0, 0, 0);
    which_color(c);

}

pub fn main() {
    pattern_matching();
}