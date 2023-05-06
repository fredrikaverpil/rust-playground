#![allow(unused)]

extern crate rand;
use rand::Rng;

pub fn main() {
    let mut rng = rand::thread_rng();
    let b: u32 = rng.gen();
}
