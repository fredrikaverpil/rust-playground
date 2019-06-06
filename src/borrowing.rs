



fn borrowing_immutable() {
    let print_vector = |x:&Vec<i32>|  // make arg type into reference of vector by prefixing it with &
    {
        println!("print_vector: {:?}", x);
        // x.push(9); // the reference is immutable, so we cannot perform something like this
    };

    let v = vec![1, 2, 3];

    print_vector(&v);  // let print_vector borrow v for a while by passing a reference of v

    println!("{:?}", v);
}

fn borrowing_mutable() {
    let mut a = 40;
    {
        let b = &mut a;
        *b += 2;
    }
    println!("a = {}", a);  // b = 42
}





pub fn main() {
    borrowing_immutable();
    borrowing_mutable();
}