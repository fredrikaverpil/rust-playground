// mutex = mutual exclusion
// threads are mutually excluded from modifying the actual variable until they are allowed to do so

#![allow(unused)]

use crate::reference_counting;
use std::rc::Rc; // Rc is not thread safe
use std::sync::{Arc, Mutex}; // Arc is thread safe
use std::thread;

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>,
}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person {
            name: name,
            state: state,
        }
    }

    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        println!("hello my name is {} and I am {}", self.name, state.as_str());
    }
}

fn mutex() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.greet(); // Rc is not thread safe and thus this will not compile!
    });
    println!(
        "Name = {}, state = {}",
        name,
        state.lock().unwrap().as_str()
    );

    t.join().unwrap();
}

pub fn main() {
    mutex();
}
