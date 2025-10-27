use std::fs;
use std::io;

fn main() {
    let mut path = String::new();

    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim();

    match fs::read(path) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }
}
