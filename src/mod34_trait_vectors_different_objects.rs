trait Animal {
    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name);
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name);
    }
}

// we now want to add both humans and cats to a vector

pub fn main() {
    let mut animals: Vec<Box<dyn Animal>> = Vec::new();

    animals.push(Box::new(Human { name: "John" }));
    animals.push(Box::new(Cat { name: "Misty" }));

    for a in animals.iter() {
        a.talk();
    }
}
