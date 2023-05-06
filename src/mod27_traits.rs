/*

A trait in Rust is a group of methods that are defined for a particular type.
Traits are an abstract definition of shared behavior amongst different types.
So, in a way, traits are to Rust what interfaces are to Java or abstract
classes are to C++.

*/

trait Car {
    fn name(&self) -> &'static str;
    fn honk(&self) {
        println!("{} says: Honk!", self.name());
    }
}

struct Volvo {
    name: &'static str,
}

struct Tesla {
    name: &'static str,
}

impl Car for Volvo {
    fn name(&self) -> &'static str {
        self.name
    }
}

impl Car for Tesla {
    fn name(&self) -> &'static str {
        self.name
    }

    fn honk(&self) {
        println!("{} says: Beep-bee-bee-boop-bee-doo-weep", self.name());
    }
}

// Example with constructor-like behavior below

trait Animal {
    fn create(name: &'static str) -> Self; // static function retuning the implementor

    fn name(&self) -> &'static str; // has no default implementation

    fn talk(&self) {
        // talk has a default implementation
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str,
}

struct Cat {
    name: &'static str,
}

// the below will not work, because we must implement all methods of the trait
// except talk, which has a default implementation
// impl Animal for Human {}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name);
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name);
    }
}

// Traits can also be used to define operators
// In this case a "sum()" operator is defined for a vector (list)

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for x in self {
            result += *x;
        }
        return result;
    }
}

fn traits() {
    let v = Volvo { name: "Volvo" };
    let t = Tesla { name: "Tesla" };
    println!("{} is a car", v.name());
    v.honk();
    println!("{} is a car", t.name());
    t.honk();

    // let h = Human {name: "John"};
    let h: Human = Animal::create("John"); // call create instead!
    h.talk();

    let c = Cat { name: "Misty" };
    c.talk();

    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum());
}

pub fn main() {
    traits()
}
