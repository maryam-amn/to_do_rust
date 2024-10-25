use chrono::NaiveDate;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::{self, read_to_string};
use std::io::{self};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    content: String,
    status: bool,
    deadline: Option<NaiveDate>,
}
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Flag {
    #[arg(long, short, default_value_t = 0)]
    // to indicate that a to-do can be delete
    delete: usize,
    // Option<usize>
    // to indicate that the to do is finished
    #[arg(long, default_value_t = 0, short = 'D')]
    done: usize, //bool
    // to indicate that the to-do is unfinshed 3 line
    #[arg(long, default_value_t = 0, short = 'u')]
    undone: usize, //bool
    // to indicate we can have a deadline
    #[arg(long)]
    due: Option<String>,

    #[arg(long, default_value_t = 0)]
    id: usize,

    #[arg(long,)]
    list: Option<bool>,
}

fn main() -> std::io::Result<()> {
    let flags = Flag::parse();

    let mut todos: Vec<Todo> = match read_to_string("todo.json") {
        Ok(file_content) => serde_json::from_str(&file_content).expect("Cannot deseraliaze Json"),
        Err(_) => Vec::new(),
    };

    // there is a  delete flag who remove a elements

    if flags.delete > 0 && flags.delete <= todos.len() {
        todos.remove(flags.delete - 1);
    } else if flags.done > 0 && flags.done <= todos.len() {
        todos[flags.done - 1].status = true;
    } else if flags.undone > 0 && flags.undone <= todos.len() {
        todos[flags.undone - 1].status = false;
    } else if let Some(due_date) = flags.due {
        match NaiveDate::parse_from_str(&due_date, "%Y-%m-%d") {
            Ok(date) => {
                todos[flags.id - 1].deadline = Some(date);
            }
            Err(_) => {
                println!("Mauvais format");
            }
        }
    } else if  flags.list.expect("Erreur de commande") {
            
            for (i, list_todo) in todos.iter().enumerate() {
            let status = if list_todo.status { "Done" } else { "Undone" };
            println!("{}. {}, {}", i + 1, list_todo.content, status);
            }
    }else {
        let mut user_input = String::new();
        println!("Enter a to-do list");
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();

        let user_todo = Todo {
            content: user_input.to_string(),
            status: false,
            deadline: None,
        };
        todos.push(user_todo);
    }

    fs::write(
        "todo.json",
        serde_json::to_string_pretty(&todos).expect("err"),
    )
    .expect("can't write");

    Ok(())
}
