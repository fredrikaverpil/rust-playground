const MEANING_OF_LIFE: u8 = 42; // no fixed address

static mut Z: i32 = 123; // unsafe operation!

pub fn main() {
    println!("{}", MEANING_OF_LIFE);

    // println!("{}", Z);  // will not compile, must place inside "unsafe" block
    unsafe {
        println!("{}", Z);
    }
}
