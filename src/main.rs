use std::fs;
use std::io;

fn main() {
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim();

    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_file() {
                println!("success");
            } else {
                println!("failure");
            }
        }
        Err(_) => {
            println!("failure");
        }
    }
}
