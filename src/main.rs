use std::fs::{self, read_to_string};
use std::io::{self};

use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    content: String,
}
fn main() -> std::io::Result<()> {
    let mut user_input = String::new();

    println!("Enter the to-do list:");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Read line failed.");

    let mut todos: Vec<Todo> = match read_to_string("todo.json") {
        Ok(file_content) => serde_json::from_str(&file_content).expect("Cannot deseraliaze Json"),
        Err(_) => Vec::new(),
    };

    let user_input = user_input.trim();

    // delete flag
    if user_input.contains("--delete") {
        let test = user_input.split(" ").last().expect("Cannot find number");
        //verifie si il y a un nombre
        let todo_number: usize = test.parse().expect("Cannot convert number");

        todos.remove(todo_number - 1);
    } else {
        todos.push(Todo {
            content: user_input.to_string(),
        });
    }
    // Write in the Json file
    fs::write("todo.json", serde_json::to_string(&mut todos).expect("err")).expect("can't write");

    Ok(())
}
