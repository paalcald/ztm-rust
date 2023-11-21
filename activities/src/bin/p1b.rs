use std::{io::Write, str::FromStr, collections::HashMap, fmt::Display};

use thiserror::Error;

const MAIN_MENU_CONTENTS: &str = include_str!("mainmenu.txt");
const MODIFY_MENU_CONTENTS: &str = include_str!("modifymenu.txt");

pub trait UserRetrievable
{
    fn get_user_input(desc: &str) -> Option<Self> where Self: Sized;
}

impl<T> UserRetrievable for T
where T: FromStr
{
    fn get_user_input(desc: &str) -> Option<T> {
        print!("{}", desc );
        std::io::stdout().flush().expect("couldn't flush");
        loop {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).expect("failed to read line");
            let buf = buf.trim();
            if buf.is_empty() {
                break None
            }
            if let Ok(input) = buf.parse::<T>() {
                break Some(input)
            } else {
                print!("Could not parse input, try again: ");
                std::io::stdout().flush().expect("couldnt flush");
            }
        }
    }
}

#[derive(Debug)]
pub struct Bill
{
    desc: String,
    amount: f64,
   // date: DateTime<Utc>
}

impl Bill {
    fn new(desc: String, amount: f64) -> Self {
        Self {
            desc,
            amount,
           // date: chrono::offset::Utc::now()
        }
    }
}
impl Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}€", self.desc, self.amount)            
    }
}

impl UserRetrievable for Bill {
    fn get_user_input(desc: &str) -> Option<Self> where Self: Sized {
        println!("Gathering {}", desc);
        println!("----------");
        std::io::stdout().flush().expect("could not flush");
        Some(Self::new(
            String::get_user_input("Input description: ")?,
            f64::get_user_input("Input purchase amount: ")?
        ))
    }
}
#[derive(Debug)]
pub struct Bills(pub Vec<Bill>);
impl Default for Bills {
    fn default() -> Self {
        Self::new()
    }
}
impl Display for Bills {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), {
            | result, bill |
            result.and_then(|_| writeln!(f, "\t{}", bill))
        })
    }
}
impl Bills {
    fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug)]
pub struct Database 
{
    bills: HashMap<String, Bills>
}
impl Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.bills.iter().fold(
            write!(f, "----------------\n%%%% Bills %%%%\n---------------\n"),
            |result, (name, bills) | 
            {
                result.and_then(|_| write!(f, "{}\n{}\n", name, bills))
        })
    }
}
impl Database {
    fn new () -> Self {
        Self { bills: HashMap::new() }
    }

    fn insert_bill(&mut self,name: &str, bill: Bill) {
        let client = self.bills.entry(name.to_owned()).or_default();
        client.0.push(bill);
    }

}
#[derive(Debug)]
enum MenuChoice {
    AddNewBill,
    ViewExistingBills,
    RemoveExistingBill,
    EditExistingBill,
    Quit,
}
#[derive(Debug, Error)]
enum MenuChoiceError {
    #[error("Integer out of range.")]
    IntegerOutOfRange,
    #[error("Can't parse input.")]
    ParseError,
}
impl FromStr for MenuChoice {
    type Err = MenuChoiceError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::AddNewBill),
            "2" => Ok(Self::ViewExistingBills),
            "3" => Ok(Self::RemoveExistingBill),
            "4" => Ok(Self::EditExistingBill),
            "5" => Ok(Self::Quit),
            other if other.parse::<usize>().is_ok() => Err(Self::Err::IntegerOutOfRange),
            _ => Err(Self::Err::ParseError),
        }
    }
}

#[derive(Debug)]
enum ModifyChoice {
    Name,
    Desc,
    Amount,
}
#[derive(Debug, Error)]
enum ModifyChoiceErr {
    #[error("Parse error")]
    ParseError,
    #[error("Index out of range")]
    IndexOutOfRange,
}
impl FromStr for ModifyChoice {
    type Err = ModifyChoiceErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::Name),
            "2" => Ok(Self::Desc),
            "3"	=> Ok(Self::Amount),
            other if other.parse::<usize>().is_ok() => Err(Self::Err::IndexOutOfRange),
            _ => Err(Self::Err::ParseError),
        }
    }
}

mod menu {
    use std::collections::HashMap;
    use crate::{Database, Bill, UserRetrievable, ModifyChoice, MODIFY_MENU_CONTENTS};

    pub fn add_new_bill(database: &mut Database) {
        if let Some(payee) = String::get_user_input("Input name: ") {
            if let Some(bill) = Bill::get_user_input("Input bill contents") {
                database.insert_bill(&payee, bill);
            }
        }
    }
    pub fn view_existing_bills(database: &mut Database) {
        println!("{}", database);
    }
    pub fn edit_existing_bill(database: &mut Database) {
        let customer_choices: HashMap<usize, String> = database.bills.keys().map(|x| x.to_owned()).enumerate().collect();
        let max_val = customer_choices.iter().len();
        customer_choices.iter().for_each(|x| println!("{}) {}", x.0, x.1));
        let customer_choice = 'validate_customer: loop {
            if let Some(choice) = usize::get_user_input("Select customer: ").filter(|x| max_val > *x) {
                break 'validate_customer choice
            }
        };
        let customer = customer_choices.get(&customer_choice).expect("input was validated").to_owned();
        let customer_bills = database.bills.get_mut(&customer).expect("input was validated");
        let max_val = customer_bills.0.len();
        customer_bills.0.iter().enumerate().for_each(|x| println!("{}) {}", x.0, x.1));
        let bill_choice = 'validate_bill: loop {
            if let Some(choice) = usize::get_user_input("Select bill: ").filter(|x| max_val > *x) {
                break 'validate_bill choice
            }
        };
        match ModifyChoice::get_user_input(MODIFY_MENU_CONTENTS) {
            Some(ModifyChoice::Name) => {
                let Some(new_customer) = String::get_user_input("New customer: ") else { return };
                match &customer_bills.0[..] {
                    [_] => {
                        let bill = customer_bills.0.pop().expect("if there is one it can be popped");
                        database.insert_bill(&new_customer, bill);

                    },
                    [_,  ..] => {
                        let bill = customer_bills.0.remove(bill_choice);
                        database.insert_bill(&new_customer, bill);
                    },
                    [] => panic!("oh fuck"),
                }
            },
            Some(ModifyChoice::Desc) => {
                if let Some(new_description) =  String::get_user_input("New description: ") {
                    customer_bills.0[bill_choice].desc = new_description
                };
            }
            Some(ModifyChoice::Amount) => {
                if let Some(new_amount) = f64::get_user_input("New amount: ") {
                    customer_bills.0[bill_choice].amount = new_amount 
                }
            }
            None => (),
        }
    }
    pub fn remove_existing_bill(database: &mut Database) {
        let customer_choices: HashMap<usize, String> = database.bills.keys().map(|x| x.to_owned()).enumerate().collect();
        let max_val = customer_choices.iter().len();
        customer_choices.iter().for_each(|x| println!("{}) {}", x.0, x.1));
        let customer_choice = 'validate_customer: loop {
            if let Some(choice) = usize::get_user_input("Select customer: ").filter(|x| max_val > *x) {
                break 'validate_customer choice
            }
        };
        let customer = customer_choices.get(&customer_choice).expect("validated input").to_owned();
        let customer_bills = database.bills.get_mut(&customer).expect("validated input");
        let max_val = customer_bills.0.len();
        let bill_choice = if max_val > 1 {
            customer_bills.0.iter().enumerate().for_each(|x| println!("{}) {}", x.0, x.1));
            'validate_bill: loop {
                if let Some(choice) = usize::get_user_input("Select bill: ").filter(|x| max_val > *x) {
                    break 'validate_bill choice
                }
            }
        } else {
                0_usize
        };
        match &customer_bills.0[..] {
            [_,  ..] => {
                customer_bills.0.remove(bill_choice);
            },
            [] => panic!("oh fuck"),
        };
    }
}

fn mainloop() -> Option<()>{
    let mut database = Database::new();
    'main: loop {

        match MenuChoice::get_user_input(MAIN_MENU_CONTENTS) {
            Some(MenuChoice::AddNewBill) => {
                menu::add_new_bill(&mut database)
            },
            Some(MenuChoice::ViewExistingBills) => {
                menu::view_existing_bills(&mut database)
            },
            Some(MenuChoice::EditExistingBill) => {
                menu::edit_existing_bill(&mut database)
            }
            Some(MenuChoice::RemoveExistingBill) => {
                menu::remove_existing_bill(&mut database)
            },
            Some(MenuChoice::Quit) => {
                println!("Legal termination");
                break 'main Some(())
            },
            None => continue 'main,
        }
    }
}

fn main() {
    if mainloop().is_some() {
        print!("done")
    };
}