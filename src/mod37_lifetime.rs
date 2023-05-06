#![allow(unused)]

// specify lifetime with astrosophy, '

// &'static str // static is a lifetime with the span of the entire program's lifetime

struct Person {
    name: String,
}

impl Person {
    fn get_ref_name(&self) -> &String {
        // fn get_ref_name<'a> (&'a self) -> &'a String {  // lifetime elision. Rust automatically makes this.
        &self.name
    }
}

struct Company<'z> {
    name: String,
    ceo: &'z Person,
}

fn lifetime() {
    /*
        if lifetime is not defined on Company and Company.ceo,
        rust will not compile as "ceo" reference may become invalid at some point.
    */
    let boss = Person {
        name: String::from("Elon Musk"),
    };
    let telsa = Company {
        name: String::from("Tesla"),
        ceo: &boss,
    };
}

fn lifetime2() {
    /*
        if lifetime is not defined in Person
    */
    let mut z: &String;
    {
        let p = Person {
            name: String::from("John"),
        };
        // z = p.get_ref_name();  // will not compile; "borrowed value does not live long enough"
    }
}

pub fn main() {
    lifetime();
    lifetime2();
}
