//! Module documentation
//!
//! # Examples
//!
//! The example must be compileable!
//!
//! ```
//! println!("here is one: {}", give_one());
//! ```
//! # Generate documentation
//!
//! Run rustdoc in terminal and this will generate a `docs` folder: `rustdoc comments_and_documentation.rs`

#![allow(unused)]

pub fn main() {

    // comment

    /*  comment block
        until closed */

    /// This function returns `1` (markdown ok!)
    fn give_one() -> i32 { 1 };
}