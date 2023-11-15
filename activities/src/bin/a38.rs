// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish
use std::thread;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let hello = thread::spawn(move || msg_hello());
    let thread = thread::spawn(move || msg_thread());
    let excited = thread::spawn(move || msg_excited());
    let vector = vec![hello,thread,excited];
    for item in vector.into_iter() {
        match item.join() {
            Ok(string) => print!("{}", string),
            Err(e) => println!("{:?}",e)
        }
    }
}
