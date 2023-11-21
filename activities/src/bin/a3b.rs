// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal
use std::cmp::Ordering;
fn main() {
    let integer = 5;
    match integer.cmp(&5) {
        Ordering::Greater => println!(">5"),
        Ordering::Less => println!("<5"),
        Ordering::Equal => println!("=5"),
    }
}
