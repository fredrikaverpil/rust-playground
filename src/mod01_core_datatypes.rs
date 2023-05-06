#![allow(unused)]

use std::mem;

pub fn main() {
    let a: u8 = 123; // unsigned (0...255), 8 bits (1 byte), immutable
    let mut b: i8 = 0; // signed (-127...128), 8 bits, mutable
    b = 42;
    let mut c = 123456789; // 32-bit signed i32

    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c)); // 8 bytes (32 bits)

    // i8 u8 i16 u16 i32 u32 i64 u64

    let z: isize = 123; // isize/usize (size of address)
    let size_of_z = mem::size_of_val(&z); // 8 bytes
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d)); // 4 bytes

    let e = 2.5; // double-precision, 8 bytes or 64 bits, f64
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e)); // 8 bytes

    let g = true;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g)); // 1 bytes

    let f = 4 > 0; // true
}
