// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Color {
    Cian,
    Rojo,
    Amarillo,
}
fn print_color(my_color: Color)
{
    let out_str = match my_color {
        Color::Cian => "Cian",
        Color::Rojo => "Rojo",
        Color::Amarillo => "Amarillo",
    };
    println!("{:?}", out_str)

}
fn main() {
    let my_color = Color::Cian;
    print_color(my_color);
}