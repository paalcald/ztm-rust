// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
enum Ticket {
    Backstage(String, f64),
    Vip(String, f64),
    Standard(f64),
}
fn main() {
    let backstage_ticket = Ticket::Backstage("Alice".to_owned(), 80.0);
    let vip_ticket = Ticket::Vip("Bob".to_owned(), 60.0);
    let standard_ticket = Ticket::Standard(25.0);
    let mut ticket_vec = Vec::new();
    ticket_vec.push(backstage_ticket);
    ticket_vec.push(vip_ticket);
    ticket_vec.push(standard_ticket);

    for ticket in &ticket_vec {
        match ticket {
            Ticket::Backstage(name, price) => {
                println!("{:?} got a backstage ticket for {:?} euros.", name, price)
            }
            Ticket::Vip(name, price) => {
                println!("{:?} got a vip ticket for {:?} euros.", name, price)
            }
            Ticket::Standard(price) => println!(
                "standard tickets are not nominal and cost {:?} euros",
                price
            ),
        }
    }
}
