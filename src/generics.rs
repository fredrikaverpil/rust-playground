#![allow(unused)]

struct Point {
    x: i32,
    y: i32
}

struct PointOption<T> {
    x: T,
    y: T
}

struct PointOptionMulti<T, V> {
    x: T,
    y: V
}

struct Line<T> {
    start: PointOption<T>,
    end: PointOption<T>
}

pub fn main() {
    let p1: Point = Point {x: 1, y: 2};
    let p2: PointOption<u32> = PointOption {x: 3, y: 4};
    let p3: PointOptionMulti<u32, f64> = PointOptionMulti {x: 5, y: 1.2};

    let lp1: PointOption<f64> = PointOption {x: 1.5, y: 2.5};
    let lp2: PointOption<f64> = PointOption {x: 4.5, y: 5.5};
    let myline = Line {start: lp1, end: lp2};
}