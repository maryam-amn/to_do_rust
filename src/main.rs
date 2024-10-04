use std::fs::File;

use std::io::{self, Write};
fn main() {
    

    let mut todo = String::new();

    println!("Veuillez faire votre to-do list :");

    let mut text = File::create("text.txt").expect("creation failed");
    
    io::stdin().read_line(&mut todo).expect("Read line failed.");
    
    // Write contents to the file
    text.write(todo.as_bytes()).expect("write failed");

   
}

