pub fn main() {
    let vec = vec![1, 2, 3];
    for x in vec {
        println!("{}", x);
    }
    // the vec variable is no longer available as it was moved into the for loop

    // the solution is to borrow it
    let vec2 = vec![4, 5, 6];
    for x in &vec2 {
        println!("{}", *x); // follow the reference
    }
    println!("{:?}", vec2); // vec2 is still available

    let vec3 = vec![7, 8, 9];
    for x in vec3.iter() {
        println!("{}", *x);
        // cannot mutate
    }

    let mut vec4 = vec![10, 11, 12];
    for x in vec4.iter_mut() {
        *x += 1; // can now mutate
        println!("{}", *x);
    }

    // reverse order
    let vec3 = vec![7, 8, 9];
    for x in vec3.iter().rev() {
        println!("in reverse {}", *x);
    }

    let vec = vec![3, 2, 1];
    let mut vec2 = vec![1, 2, 3];
    // extend vec2 and don't care about what happens with vec
    vec2.extend(vec); // under the hood, this executes vec.into_iter() and moves vec
    println!("{:?}", vec2);
}
