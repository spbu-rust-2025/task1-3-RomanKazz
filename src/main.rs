use std::fs;
use std::io;

fn main() {
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim();

    if fs::metadata(path).is_ok() {
        println!("success");
    } else {
        println!("failure");
    }
}
