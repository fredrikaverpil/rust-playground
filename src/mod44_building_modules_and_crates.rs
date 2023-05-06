extern crate phrases; // our own crate
extern crate rand; // downloaded crate

use phrases::greetings::french;
use rand::Rng;

pub fn main() {
    let mut rng = rand::thread_rng();
    let random_number: f32 = rng.gen();
    println!("Random number {}", random_number);

    println!(
        "English: {}, {}",
        phrases::greetings::english::hello(),
        phrases::greetings::english::bye()
    );

    println!("French: {}, {}", french::hello(), french::bye());
}
