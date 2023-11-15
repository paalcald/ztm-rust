// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors
#[derive(Debug)]
struct Truck;
impl Body for Truck {

}

#[derive(Debug)]
struct Car;
impl Body for Car {

}

#[derive(Debug)]
struct Scooter;
impl Body for Scooter {

}

#[derive(Debug)]
struct White;
impl Color for White {

}
#[derive(Debug)]
struct Red;
impl Color for Red {

}
#[derive(Debug)]
struct Black;
impl Color for Black {

}

#[allow(dead_code)]
#[derive(Debug)]
struct Vehicle<T, U> 
where 
    T: Body,
    U: Color{
body: T,
color: U,
}
impl <B,C> Vehicle<B,C> 
where
    B: Body,
    C: Color,
{
    fn new(body: B, color: C) -> Self {
        Self {body, color}
    }

}

trait Body {}

trait Color {}

fn main() {
    let red_car = Vehicle::new(Car, Red);
    let white_scooter = Vehicle::new(Scooter, White);
    let black_truck = Vehicle::new(Truck, Black);
    println!("{:?}", red_car);
    println!("{:?}", white_scooter);
    println!("{:?}", black_truck);

}
