// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
struct Dimensions {
    length: f64,
    width: f64,
    height: f64,
}
impl Dimensions {
    fn show(&self) -> String {
        format!("{:?}x{:?}x{:?}cm", self.length, self.width, self.height)
    }
}
enum Color {
    Yellow,
    Green,
    Blue,
    Red,
}
impl Color {
    fn show(&self) -> String {
        match self {
            Color::Yellow => String::from("yellow"),
            Color::Green => String::from("green"),
            Color::Blue => String::from("blue"),
            Color::Red => String::from("red"),
        }
    }
}
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn show(&self) {
        println!(
            "{:?} sized {:?} box weighting {:?} kg",
            self.dimensions.show(),
            self.color.show(),
            self.weight
        );
    }
    fn small_box() -> Self {
        Self {
            dimensions: Dimensions {
                length: 24.0,
                width: 17.0,
                height: 8.7,
            },
            weight: 1.0,
            color: Color::Green,
        }
    }
}
fn main() {
    let small_box = ShippingBox::small_box();
    small_box.show()
}
