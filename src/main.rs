use std::fs;
use std::io;

fn main() {
    let mut path = String::new();

    if io::stdin().read_line(&mut path).is_err() {
        println!("failure");
        return;
    }

    let path = path.trim();

    if fs::read_to_string(path).is_ok() {
        println!("success");
    } else {
        println!("failure");
    }
}
