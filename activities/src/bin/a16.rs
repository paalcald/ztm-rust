// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
#[allow(dead_code)]
struct Student {
    name: String,
    locker: Option<u32>,
}
impl Student {
    fn print_locker(&self) {
        match self.locker {
            Some(locker_id) => println!("locker {:?} assigned.", locker_id),
            None => println!("no locker assigned."),
        }
    }
}
fn main() {
    let stdnt1 = Student {
        name: "Alice".to_owned(),
        locker: Some(345),
    };
    let stdnt2 = Student {
        name: "Bob".to_owned(),
        locker: Some(325),
    };
    let stdnt3 = Student {
        name: "Chandler".to_owned(),
        locker: None,
    };

    stdnt1.print_locker();
    stdnt2.print_locker();
    stdnt3.print_locker();
}
