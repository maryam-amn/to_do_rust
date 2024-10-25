use chrono::NaiveDate;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::{self, read_to_string};
use std::io::{self};

#[derive(Debug, Serialize, Deserialize, Ord, Eq, PartialOrd, PartialEq)]
struct Todo {
    content: String,
    status: bool,
    deadline: Option<NaiveDate>,
}
#[derive(Parser)]
#[command(version, about, long_about = None)]
// we use the clap library in order to be able to automatically manage the arguments and flags of the
//command line.
struct Flag {
    #[arg(long, short, default_value_t = 0)]
    // to indicate that a to-do can be delete
    delete: usize,
    // to indicate that the to do is finished
    #[arg(long, default_value_t = 0, short = 'D')]
    done: usize, 
    // to indicate that the to-do is unfinshed 3 line
    #[arg(long, default_value_t = 0, )]
    undone: usize, 
    // to indicate we can have a deadline
    #[arg(long)]
    due: Option<String>,
    // to indicate id, or the line of the to-do
    #[arg(long, default_value_t = 0)]
    id: usize,
    // to do a list of the to-do
    #[arg(long)]
    list: bool,
    // to sort the to-do by order of priority
    #[arg(short, long)]
    sort: bool,
}

fn main() -> std::io::Result<()> {
    let flags = Flag::parse();

    let mut todos: Vec<Todo> = match read_to_string("todo.json") {
        Ok(file_content) => serde_json::from_str(&file_content).expect("Cannot deseraliaze Json"),
        Err(_) => Vec::new(),
    };
    // Here the delete flag remove a elements
    if flags.delete > 0 && flags.delete <= todos.len() {
        todos.remove(flags.delete - 1);
    }
    // there is a flage done who indicate that the task is completed
    else if flags.done > 0 && flags.done <= todos.len() {
        todos[flags.done - 1].status = true;
    }
    // the flag undone indicate that the task is not done
    else if flags.undone > 0 && flags.undone <= todos.len() {
        todos[flags.undone - 1].status = false;
    }
    // The flag due indicate a deadline
    else if let Some(due_date) = flags.due {
        match NaiveDate::parse_from_str(&due_date, "%Y-%m-%d") {
            Ok(date) => {
                todos[flags.id - 1].deadline = Some(date);
            }
            Err(_) => {
                println!("Invalid format, try again");
            }
        }
    }
    // the flag list, show us the list of  all the to-do and their status
    else if flags.list {
        for (i, list_todo) in todos.iter().enumerate() {
            let status = if list_todo.status { "Done" } else { "Undone" };
            println!("{}. {}, {}", i + 1, list_todo.content, status);
        }
    }
    // the flag sort, it sorts the to-do list in priority order
    else if flags.sort {
        todos.sort_by(|a, b| a.deadline.cmp(&b.deadline));
    }
    // if there is no flag, the input of the user will be written in the to-do list
    else {
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

    // Here the input of the user will be written in the todo json file
    fs::write(
        "todo.json",
        serde_json::to_string_pretty(&todos).expect("err"),
    )
    .expect("can't write");

    Ok(())
}
