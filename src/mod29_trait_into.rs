struct Person {
    name: String,
}

impl Person {
    fn new(name: &str) -> Person {
        Person { name: name.into() }
    }

    fn talk(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

struct Person2 {
    name: String,
}

impl Person2 {
    fn new<S: Into<String>>(name: S) -> Person2 {
        Person2 { name: name.into() }
    }
    fn talk(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

struct Person3 {
    name: String,
}

impl Person3 {
    // this is the Into trait
    fn new<S: Into<String>>(name: S) -> Person3 {
        Person3 { name: name.into() }
    }
    fn talk(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

pub fn main() {
    let john = Person::new("John");
    john.talk();

    // but what is we already have the name in a variable?
    let name = "Jane".to_string();
    let jane = Person2::new(&name);
    jane.talk();

    // if we don't want to pass as reference
    let jane = Person3::new(name);
    jane.talk();
}
