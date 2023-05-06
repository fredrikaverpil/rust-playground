#![allow(dead_code)]
#![allow(unused)]

/*

A drop trait is a destructur. Suppose you have a game, involves creatures
that gets destroyed, and you want to know when they are created or destroyed.

 */

struct Creature {
    name: String,
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

pub fn main() {
    let goblin = Creature::new("Jeff");
    println!("Game proceeds");

    // the goblin is cleaned up here
}
