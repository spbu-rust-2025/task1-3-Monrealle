use std::fs;
use std::io;

fn main() {
    let mut path = String::new();

    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim();

    match fs::read_to_string(path) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }
}
