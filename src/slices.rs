// slice is a part of an array
// unlike an array, its size is unknown

fn slices() {
    let mut data = [1, 2, 3, 4, 5];

    use_slice(&mut data[1..4]);
    // use_slice(&mut data);
}

fn use_slice(slice: &mut [i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());

    slice[0] = 4321;
}

pub fn main() {
    slices()
}
