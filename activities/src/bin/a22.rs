// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.
#[allow(dead_code)]
/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

#[allow(dead_code)]
/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    match b {
        0 => None,
        other => Some(a / other),
    }
}
#[allow(dead_code)]
/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}
#[cfg(test)]
mod test {
    use crate::{clamp, div, concat};

    #[test]
    fn clamp_lt_lower_is_lower () {
        let n = 10;
        let lower = 100;
        let upper = 1000;
        let out = clamp(n, lower, upper);
        assert_eq!(out, lower, "lower must surpass output");
    }

    #[test]
    fn clamp_gt_upper_is_upper () {
        let n = 10;
        let lower = 100;
        let upper = 1000;
        let out = clamp(n, lower, upper);
        assert_eq!(out, lower, "value must never surpass upper.");
    }
    #[test]
    fn check_zero_div() {
        let a = 10;
        let b = 0;
        let out = div(a, b);
        assert_eq!(out, None, "Zero division should return `None`");
    }
    #[test]
    fn check_div() {
        let a = 10;
        let b = 2;
        let out = div(a,b);
        assert_eq!(out, Some(5), "Should be 5");
    }
    #[test]
    fn check_concat() {
        let a = "a";
        let b = "b";
        let out = concat(a, b);
        assert_eq!(out, "ab".to_owned());
    }
}
