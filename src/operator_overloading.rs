use std::ops::Add;


#[derive(Debug)]
struct Point {
    pub x: f64,
    pub y: f64
}

impl Add for Point {

    type Output = Point;

    fn add(self, other: Point) -> Point
    {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

fn oo() {
    let p1 = Point {x: 1.0, y: 2.0};
    let p2 = Point {x: 1.0, y: 2.0};
    let p3 = p1 + p2;
    println!("{:?}", p3);
}

pub fn main() {
    oo()
}