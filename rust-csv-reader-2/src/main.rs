mod csv_read_stdin;
mod csv_read_file;
mod csv_read_serde;
mod csv_write_stdout;
mod csv_write_file;
use csv_read_stdin::{read_using_stdin};
use csv_read_file::{read_using_file};
use csv_read_serde::{read_serde_file};
use csv_write_stdout::{write_to_stdout};
use csv_write_file::{write_to_file};
use std::io;

fn main() {
    println!("(1) Read From Stdin");
    println!("(2) Read From File");
    println!("(3) Read File Using Serde");
    println!("(4) Write to Stdout");
    println!("(5) Write to File");
    let mut operation: String = String::new();
    
    io::stdin().read_line(&mut operation).expect("Invalid Input"); 

    let op: i32 = match operation.trim().parse() {
        Ok(num) => num,
        Err(_) =>  {
            println!("Invalid input!");
            return;
        }
    };
    match op {
        1 => read_using_stdin(),
        2 => read_using_file(),
        3 => read_serde_file(),
        4 => write_to_stdout(),
        5 => write_to_file(),
        _ => {
            println!("Invalid selection!");
            return;
        }
    };

}
