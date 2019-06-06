#![allow(unused)]

use crate::reference_counting;
use std::rc::Rc;  // Rc is not thread safe
use std::sync::Arc;  // Arc is thread safe
use std::thread;

struct PersonArc {
    name: Arc<String>
}

impl PersonArc {
    fn new(name: Arc<String>) -> PersonArc {
        PersonArc { name: name }
    }

    fn greet(&self) {
        println!("hello my name is {}", self.name);
    }
}

// fn rc() {
//     let name = Rc::new("John".to_string());
//     let person = reference_counting::PersonRc::new(name);
//     let t = thread::spawn(move || {
//         person.greet();  // Rc is not thread safe and thus this will not compile!
//     });
//     println!("Name = {}", name);

//     t.join().unwrap();
// }

fn arc() {
    let name = Arc::new("John".to_string());
    let person = PersonArc::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}", name);

    t.join().unwrap();
}

pub fn main() {
    // rc()
    arc()
}