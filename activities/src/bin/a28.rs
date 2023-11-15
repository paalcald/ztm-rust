// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing
#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}
#[derive(Debug)]
struct Shirt(Color);
impl Shirt {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct Pants(Color);
impl Pants {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct Shoes(Color);
impl Shoes {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}


fn main() {
    let shoes = Shoes::new(Color::Black);
    let pants = Pants::new(Color::Blue);
    let shirt = Shirt::new(Color::Red);
    println!("{:?}", shoes);
    println!("{:?}", pants);
    println!("{:?}", shirt);
}