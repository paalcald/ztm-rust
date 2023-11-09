use std::{collections::HashMap, hash::Hash, io::{Write, self}};

// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
#[derive(Clone)]
struct Bill {
    name: String,
    amount: f64,
}
impl Bill {
    fn new(name: String, amount: f64) -> Self {
        Self {name, amount}
    }
    fn display_bill(&self) -> String{
        format!("{}: {}.", self.name, self.amount)
    }
}

struct Database {
    bills: Vec<Bill>,
    variables: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        Self {
            bills: Vec::new(),
            variables: HashMap::new(),
        }
    }
    fn add_bill (&mut self, new_bill: Bill) {
        self.bills.push(new_bill);
    }

    fn modify_bill_name(&mut self, index: usize, new_name: String) -> Result<(), Sring> {
        if index < self.bills.len() - 1 {
            self.bills.get_mut(index).name = new_name;
            Ok(())
        } else {
            Err("Wrong input size".to_owned())
        }
    }

    fn modify_bill_amount(&mut self, index: usize, new_amount: f64) -> Result<(), Sring> {
        if index < self.bills.len() {
            self.bills.get_mut(index).amount = new_amount;
            Ok(())
        } else {
            Err("Wrong input size".to_owned())
        }
    }

    fn remove_bill(&mut self, index: usize) -> Result<(), String> {
        if index < self.bills.len() {
            self.bills.remove(index);
            Ok(())
        } else {
            Err("Wrong input size".to_owned())
        }
        
    }

    fn get_all (&self) -> std::slice::Iter<'_, Bill> {
        self.bills.iter()
    }
}
pub mod cli {
    use std::collections::HashMap;
    use std::io;
    use std::io::Write;

    #[derive(PartialEq, Eq, Hash)]
    enum CLICommand {
        AddNewBill,
        ViewBills,
    }
    
    pub fn run(&) -> Optional 


    pub fn update_interface(&self) {
        ()
    }

    pub fn get_input() -> Option<String> {
        let mut buffer = String::new();
        while io::stdin().read_line(&mut buffer).is_err() {
            print!("\nCould not read input format.\nTry again: ");
            io::stdout().flush().expect("couldnt flush");
        }
        if buffer.is_empty() {
            None
        } else {
            Some(buffer)
        }
    } 
}
fn main() {}
#[cfg(test)]
mod test {
    use crate::Bill;

    #[test]
    fn check_new_bill() {
        let my_bill = Bill::new("Default".to_owned(), 12.3);
        assert_eq!("Default".to_owned(), my_bill.name, "Default expected.");
        assert_eq!(12.3, my_bill.amount, "Expected 12.3");
    }
    #[test]
    fn check_display_bill() {
        let my_bill = Bill {name: "Default".to_owned(), amount: 10.0};
        assert_eq!("Default: 10.".to_owned(), my_bill.display_bill(), "expected \"Default\": 10.");
    }
    use crate::Database;

    #[test]
    fn check_new_db() {
        let my_db = Database::new();
        assert!(my_db.bills.is_empty());
        assert!(my_db.variables.is_empty());
    }
    #[test]
    fn check_add_bill_db() {
        let mut my_db = Database::new();
        let my_bill = Bill::new("Default".to_owned(), 12.3);
        let my_backup_bill = my_bill.clone();
        my_db.add_bill(my_bill);
        let item = my_db.bills.pop().unwrap();
        assert_eq!(my_backup_bill.name, item.name, "Expected name.");
        assert_eq!(my_backup_bill.amount, item.amount, "Expected amount owed.");
    }
    
    #[test]
    fn check_get_all_db() {
        let mut my_db = Database::new();
        let my_bill = Bill::new("Default".to_owned(), 12.3);
        let my_backup_bill = my_bill.clone();
        my_db.add_bill(my_bill);
        let item = my_db.get_all().next().unwrap();
        assert_eq!(my_backup_bill.name, item.name, "Expected name.");
        assert_eq!(my_backup_bill.amount, item.amount, "Expected amount owed.");
    }
}
