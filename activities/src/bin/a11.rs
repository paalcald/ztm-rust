// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
struct Item {
    id_number: i32,
    quantity: i32,
}
fn get_id(item: &Item) {
    println!("id_number: {:?}", item.id_number);
}
fn get_quantity(item: &Item) {
    println!("quantity: {:?}", item.quantity);
}
fn main() {
    let item = Item {
        id_number: 123,
        quantity: 10,
    };
    get_id(&item);
    get_quantity(&item);
}
