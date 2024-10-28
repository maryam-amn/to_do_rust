# Rust - To do list in command in line 


## Install  rust

## macOs, Linux or Unix-like OS

###  If you're using a laptoop who's  running running macOs, Linux or Unix-like OS. To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions. 


     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## Window and Unix

### On Unix,
 run the following in your shell, this downloads and runs rustup-init.sh, which in turn downloads and runs the correct version of the rustup-init executable for your platform.

    run curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 


  ### On Windows
   download and run  [rustup-init.exe. ](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe) rustup-init can be configured interactively, and all options can additionally be controlled by command-line arguments, 
   
   which can be passed through the shell script. Pass --help to rustup-init as follows to display the arguments rustup-init accepts:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --help

for more information : 
-  [Rust - website ](https://www.rust-lang.org/tools/install)

## Commande Rust 

Allows you to create a project
- `  Cargo new ` ` exemple_new`


## To-do list  documentation

>Document json

To read and write in the to-do list, I used a Json file because, JSON is a text format for storing and transporting data.It is easy for humans to read and write and it is easy for machines to parse and generate.

>clap library 

This dependency allows us to use flags in the command line.

To add the Clap dependencies to your Rust project, run the following command in your terminal.

`cargo add clap --features derive`

>chrono
This dependency allows us to use a date format in the code .
To add the Clap dependencies to your Rust project, run the following command in your code.

`chrono ={ version = "0.4.38", features = ["serde"]}`
