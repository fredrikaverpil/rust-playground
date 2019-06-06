extern crate rand;  // downloaded crate
extern crate phrases;  // our own crate

use rand::Rng;
use phrases::greetings::french;

pub fn main() {

    let mut rng = rand::thread_rng();
    let random_number: f32 = rng.gen();
    println!("Random number {}", random_number);

    println!("English: {}, {}",
        phrases::greetings::english::hello(),
        phrases::greetings::english::bye());

    println!("French: {}, {}",
        french::hello(),
        french::bye());

}