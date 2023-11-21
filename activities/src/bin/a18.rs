// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message
struct Adult {
    name: String,
    age: u8,
}
impl Adult {
    fn new(name: String, age: u8) -> Result<Self, String> {
        if age > 21 {
            Ok(Adult { name, age })
        } else {
            Err(format!("Try again in {:?} years.", 21 - age))
        }
    }
}
fn main() {
    let kid = Adult::new("kiddo".to_owned(), 10);
    let grampa = Adult::new("aurelio".to_owned(), 80);
    let people = vec![kid, grampa];
    for pep in &people {
        match pep {
            Ok(adult) => println!("{:?} is {:?}.", adult.name, adult.age),
            Err(msg) => println!("{:?}", msg),
        }
    }
}
