use std::mem;

fn array() {
    // NOTE: arrays cannot be resized

    // Add brackets and the length in the declaration
    let mut a:[i32;5] = [1, 2, 3, 4, 5];

    a[0] = 321;

    println!("a has {} elements, first is {}", a.len(), a[0]);

    println!("{:?}", a);  // debug print will print the entire array

    if a != [1, 2, 3, 4, 5] {
        println!("does not match");
    }

    let b = [1; 10];  // add 10 elements with the value of 1

    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));  // 40 bytes

    let c = [1u16; 10];
    println!("c took up {} bytes", mem::size_of_val(&c));  // 20 bytes

    // matrix
    let mtx:[[f32;3]; 2] =
    [
        // rows
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];

    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

pub fn main() {
    array();
}