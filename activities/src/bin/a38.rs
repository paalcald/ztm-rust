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
    let hello = thread::spawn(msg_hello);
    let thread = thread::spawn(msg_thread);
    let excited = thread::spawn(msg_excited);
    let vector = [hello, thread, excited];
    for item in vector {
        match item.join() {
            Ok(string) => print!("{}", string),
            Err(e) => println!("{:?}",e)
        }
    }
}
