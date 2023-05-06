#![allow(unused)]

use std::rc::Rc;

struct Person {
    name: String,
}

impl Person {
    fn new(name: String) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("hello my name is {}", self.name);
    }
}

fn without_rc() {
    let name = "John".to_string();
    let person = Person::new(name);

    // println!("name = {}", name)  // does not compile, because using moved value for "name"
}

pub struct PersonRc {
    name: Rc<String>,
}

impl PersonRc {
    pub fn new(name: Rc<String>) -> PersonRc {
        PersonRc { name: name }
    }

    pub fn greet(&self) {
        println!("hello my name is {}", self.name);
    }
}

fn with_rc() {
    let name = Rc::new("John".to_string());
    println!(
        "Name = {}, name has has {} strong pointers",
        name,
        Rc::strong_count(&name)
    ); // 1 strong pointer
    {
        let person = PersonRc::new(name.clone()); // increase the reference count with .clone()
        println!(
            "Name = {}, name has has {} strong pointers",
            name,
            Rc::strong_count(&name)
        ); // 2 strong pointers
        person.greet()
    }
    println!(
        "Name = {}, name has has {} strong pointers",
        name,
        Rc::strong_count(&name)
    ); // 1 strong pointer
}

pub fn main() {
    without_rc();
    with_rc();
}
