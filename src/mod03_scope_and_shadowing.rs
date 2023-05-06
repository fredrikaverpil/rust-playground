pub fn main() {
    let _a = 123;
    {
        // new scope!
        let _b = 456;

        let _a = 0;  // shadowing of _a in this scope only
    }
    // _b not available here!
}