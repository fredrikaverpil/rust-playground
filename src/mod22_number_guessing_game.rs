#![allow(dead_code)]

use rand::Rng; // random
use std::io::stdin; // whatever you type into the console

fn variant1() {
    let number = rand::thread_rng().gen_range(1, 101); // random number between 1 and 100
    loop {
        println!("Guess the number!");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range!")
                        } else if guess < number {
                            println!("Too small!");
                        } else if guess > number {
                            println!("Too big!");
                        } else {
                            println!("You win!");
                            break;
                        }
                    }
                    Err(_) => continue, // continue the loop
                }
            }
            Err(_) => continue, // some kind of error, continue the loop
        }
    }
}

fn variant2() {
    let number = rand::thread_rng().gen_range(1, 101); // random number between 1 and 100
    loop {
        println!("Guess the number!");
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Failed to read line");
        let guess: u32 = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // continue the loop
        };
        match guess.cmp(&number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

pub fn main() {
    println!("Guessing game time!");
    // variant1();
    // variant2();
}
