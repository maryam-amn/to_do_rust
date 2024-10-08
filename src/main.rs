use std::io::{self, Write};

use std::fs::OpenOptions;

fn main() {
    let mut todo = String::new();

    println!("Veuillez faire votre to-do list :");

    // Open the file 
    let mut text = OpenOptions::new()
        .append(true)
        .create(true)
        .open("text.txt")
        .expect("cannot open file");

    io::stdin().read_line(&mut todo).expect("Read line failed.");

    let todo: &str = todo.trim();

    println!("message ");
    writeln!(text, "{}", todo).expect("cannot open file");
}
