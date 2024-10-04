use std::fs::File;

use std::io::{self, Write};
fn main() {
    

    let mut todo = String::new();

    let mut todo2 = String::new();


    println!("Veuillez faire votre to-do list :");

    let mut text = File::create("text.txt").expect("creation failed");
    // prend le inpu et

    io::stdin().read_line(&mut todo).expect("Read line failed.");

    // Write contents to the file
    text.write(todo.as_bytes()).expect("write failed");


    // Ajouter une to do 
    io::stdin().read_line(&mut todo2).expect("Read line failed.");

    text.write(todo2.as_bytes()).expect("write failed");

    
}



