use std::fs::{self, read_to_string};
use std::io::{self};
use serde::{Deserialize, Serialize};
use serde_json;
use clap::Parser;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    content: String,
}
#[derive(Parser,Debug)]
#[command(version = "0.1")]
struct Flag {
    #[arg(long, short)]
    delete: Option<usize>
}
fn main() -> std::io::Result<()> {
    
    let flags = Flag::parse();

    let mut todos: Vec<Todo> = match read_to_string("todo.json") {
        Ok(file_content) => serde_json::from_str(&file_content).expect("Cannot deseraliaze Json"),
        Err(_) => Vec::new(),
    };

    // delete flag
  
    if let Some(number_line) = flags.delete {
        if number_line > 0 && number_line <= todos.len() {
            todos.remove(number_line - 1);
        } 
    } else {
        let mut user_input = String::new();
        println!("Enter a to-do list");
        io::stdin().read_line(&mut user_input)?;

        let user_input = user_input.trim();
        if !user_input.is_empty() {
            todos.push(Todo {
                content: user_input.to_string(),
            });
            } 
    }
    
    fs::write("todo.json", serde_json::to_string(&mut todos).expect("err")).expect("can't write");

    Ok(())
}