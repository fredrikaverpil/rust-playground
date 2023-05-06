#![allow(dead_code)]
#![allow(unused)]

union IntOrFloat {
    // 32-bits
    // Can store _either_ an i32 or an f32, but not both at the same time
    i: i32,
    f: f32,
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life");
            }
            IntOrFloat { f } => {
                println!("f32 = {}", f);
            }
        }
    }
}

fn unions() {
    let mut iof = IntOrFloat { i: 123 }; // it would be unsafe to modify this union

    // we don't know if this is an int or float, so we have to use an "unsafe" block
    unsafe {
        iof.i = 42;
    }

    let value = unsafe { iof.i };

    process_value(iof);
    process_value(IntOrFloat { f: 1.23 });
}

pub fn main() {
    unions();
}
