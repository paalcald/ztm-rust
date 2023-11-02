// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
fn sub<T: std::ops::Sub<Output = T>>(a: T, b: T) -> T {
    a - b
}
fn display_result<T: std::fmt::Debug>(result: T) {
    println!("{:?}", result);
}
fn main() {
    let a = 1;
    let b = 2;
    let result = add(a, b);
    display_result(result);
}

