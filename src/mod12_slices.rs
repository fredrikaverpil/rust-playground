fn use_slice(slice: &mut [i32]) {
    // the & in the signature means we are borrowing a part of the array (a slice)
    println!("first elem = {}, len = {}", slice[0], slice.len());

    slice[0] = 4321;
}

fn slices() {
    // slice is a part of an array
    // unlike an array, its size is unknown
    let mut data = [1, 2, 3, 4, 5];

    use_slice(&mut data[1..4]);
    // use_slice(&mut data);  // you can also pass an entire array

    println!("{:?}", data); // [1, 4321, 3, 4, 5]
}

pub fn main() {
    slices()
}
