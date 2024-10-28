# User instruction
## How to use the to-do list ? 

To add, simply run `cargo run` and the program will ask for the tasks you want to add to the to-do list.

This project is a to-do list that allows us to put to-dos in a file. We can use different flags, which are useful in a to-do list: 

The different flags and what they can do  : 

    -- --delete flag the draper followed by the to-do number deletes a todo
    
    -- --done flag followed by the todo number to indicate that it is completed.

    -- --undone flag followed by the todo number to indicate that it is not completed.

    -- --due flag followed by a date in the format "YY-MM-DD" to add a deadline.

    -- --list flag to display all todos and their status (done, undone).

    -- --sort flag to sort the list in order of priority.

## example 

`cargo run` 

`cargo run -- --delete 1`

`cargo run --done 2`

`cargo run -- --undone 1` 

`cargo run -- --undone 1` 

`cargo run -- --due 2024-12-13 --id 1`

`cargo run -- --list 1` 

`cargo run -- --sort`





