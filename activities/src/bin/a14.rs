// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name[] and colors should be printed using a function
enum Color {
    Yellow,
    Cian,
    Red,
}

impl Color {
    fn show(&self) -> String {
        match self {
            Color::Yellow => "yellow".to_owned(),
            Color::Cian => "cian".to_owned(),
            Color::Red => "red".to_owned(),
        }
    }
}
struct Person {
    age: i32,
    name: String,
    color: Color,
}
impl Person {
    fn new(age: i32, name: String, color: Color) -> Self {
        Self { age, name, color }
    }
    fn show(&self) -> String {
        std::format!(
            "{:?} tiene {:?} años y su color favorito es el {:?}",
            self.name,
            self.age,
            self.color.show()
        )
    }
}
fn main() {
    let my_people = vec![
        Person::new(10, "Paco".to_owned(), Color::Cian),
        Person::new(30, "Paloma".to_owned(), Color::Yellow),
        Person::new(35, "Pablo".to_owned(), Color::Red),
    ];
    for dude in &my_people {
        if dude.age <= 10 {
            println!("{:?}", dude.show());
        }
    }
}
