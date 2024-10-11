use std::fs::{self, read_to_string};
use std::io::{self};

fn main() -> std::io::Result<()> {
    let mut todo = String::new();

    println!("Enter the to-do list:");
    io::stdin().read_line(&mut todo).expect("Read line failed.");

    let todo: &str = todo.trim();

    let mut todos: Vec<String> = match read_to_string("text.txt") {
        Err(_) => Vec::new(),
        Ok(text) => text.lines().map(String::from).collect(),
    };

    if todo.contains("--delete") {
        let test = todo.split("pat").last();
        let number_line: usize = test.expect("Err").parse().unwrap();

        todos.remove(number_line - 1);
    } else {
        todos.push(todo.to_string())
    };

    fs::write("text.txt", todos.join("\n")).expect("can't write");

    Ok(())
}
