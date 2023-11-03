// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavour {
    Orange,
    Strawberry,
    Pineaple,
    Cola,
}

struct Drink {
    flavour: Flavour,
    size: i32,
}
fn print_drink(d: Drink) {
    let d_flavour = match d.flavour {
        Flavour::Orange => "orange",
        Flavour::Strawberry => "strawberry",
        Flavour::Pineaple => "pineaple",
        Flavour::Cola => "cola",
    };

    println!(
        "You beberage favour is {} and is {:?} fluid ounces.",
        d_flavour, d.size
    );
}
fn main() {
    let cola8oz = Drink {
        flavour: Flavour::Cola,
        size: 8,
    };
    print_drink(cola8oz)
}
