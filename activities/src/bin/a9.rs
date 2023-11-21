// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print
use std::cmp::Ordering;

fn coordinate() -> (i32, i32) {
    (123, 5)
}
fn main() {
    let my_point = coordinate();
    let (_, y) = my_point;
    match y.cmp(&5) {
        Ordering::Greater => println!(">5"),
        Ordering::Less => println!("<5"),
        Ordering::Equal => println!("=5"),
    }
}
