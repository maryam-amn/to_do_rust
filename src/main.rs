use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, read_to_string};
use std::io::{self};
use std::usize;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    content: String,
    status: bool,
}
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Flag {
    #[arg(long, short, default_value_t = 0)]
    delete: usize,
    // to indicate that the to do is finished
    #[arg(long, default_value_t = 0)]
    done: usize,
}

fn main() -> std::io::Result<()> {
    let flags = Flag::parse();

    let mut todos: Vec<Todo> = match read_to_string("todo.json") {
        Ok(file_content) => serde_json::from_str(&file_content).expect("Cannot deseraliaze Json"),
        Err(_) => Vec::new(),
    };

    // delete flag

    if flags.delete > 0 && flags.delete <= todos.len() {
        todos.remove(flags.delete - 1);
    } else if flags.done > 0 && flags.done <= todos.len() {
        todos[flags.done - 1].status = false;
    } else {
        let mut user_input = String::new();
        println!("Enter a to-do list");
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();

        let user_todo = Todo {
            content: user_input.to_string(),
            status: false,
        };
        todos.push(user_todo);
    }

    fs::write("todo.json", serde_json::to_string(&mut todos).expect("err")).expect("can't write");

    Ok(())
}
