#![allow(unused_variables)]

fn strings() {
    // there are two string types

    // statically allocated, included in our compiled program, not created during runtime
    // cannot be re-assigned, cannot be manipulated
    let s: &'static str = "hello there"; // str = string slice
                                         // let h = s[0];  // will not work, as you would have to index the bytes...

    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char)
    }

    // heap allocated construct "String"
    let mut letters = String::new();

    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // conversion between &str and String
    let u: &str = &letters;

    // concatenation
    // String + str
    let z = letters + "abc";
    let k = String::from("hello world");
    let mut l = "hello world".to_string();
    l.remove(0);
    l.push_str("!!!");
    println!("{}", l.replace("ello", "goodbye"));
}

pub fn main() {
    strings();
}
