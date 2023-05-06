use std::collections::HashMap;

// like dict in python

pub fn main() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    // will print "a square has 4 sides"
    println!("a square has {} sides", shapes["square".into()]);

    // will print {"triangle": 3, "square": 4}
    println!("{:?}", shapes);

    for (key, value) in &shapes {
        println!("{}: {}", key, value);
    }

    // overwriting a value
    shapes.insert("square".into(), 5);

    // insert key/value only if the key does not already exist
    shapes.entry("triangle".into()).or_insert(1); // does not overwrite the value for "triangle"

    shapes.entry("circle".into()).or_insert(360); // adds a new key/value for "circle"

    // get access to the current value and update it
    {
        let actual = shapes.entry("circle".into()).or_insert(0); // reference to actual value
        *actual = 360;
    }

    // remove key-value pair
    shapes.remove("square");

    println!("{:?}", shapes);
}
