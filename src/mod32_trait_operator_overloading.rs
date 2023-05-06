use std::ops::Add; // the Add trait

#[derive(Debug)]
struct Point {
    pub x: f64,
    pub y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn operator_overloading() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = p1 + p2;
    println!("{:?}", p3); // Point { x: 2.0, y: 4.0 }
}

pub fn main() {
    operator_overloading()
}
