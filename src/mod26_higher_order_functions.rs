// higher order functions
//
// functions that take functions:
// f(g) { let x = g(); }
//
// and/or functions that return functions (sometimes called generators):
// f() -> g

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

// implements a function trait
// the limit does not live long enough, hence the "move"
fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    // return a function
    move |y| y > limit
}

fn sum_of_all_even_squares_are_less_than_500() {
    let limit = 500;
    let mut sum = 0;

    // let above_limit = |y| y > limit;
    let above_limit = greater_than(limit);

    for i in 0.. {
        let isq = i * i;

        // if isq > limit {
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x * x) // take the value of x and multiply it with x
        .take_while(|&x| x <= limit) // take the (reference of the) value of x and check if it is less than or equal to limit
        .filter(|x: &u32| is_even(*x)) // take the (deferenced) value of x and check if it is even
        .fold(0, |sum, x| sum + x); // use 0 as initial value of accumulator, add the arguments to
                                    // the accumulator and return the sum of the accumulator
    println!("loop sum2 = {}", sum2);
}

pub fn main() {
    sum_of_all_even_squares_are_less_than_500()
}
