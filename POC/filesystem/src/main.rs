use std::io;
use std::io::*;
#[allow(non_snake_case)]

pub struct File {
    name: String,
    full_type: String,
}

fn main() {
    /*Variables*/
    let run: bool = true;
    let mut input = String::new();
    /*On Start-Up*/
    println!("  DEMO SHELL");
    /*Main program*/
    while run {
        print!("> ");
        io::stdout().flush().unwrap();
        stdin().read_line(&mut input).ok().expect("Error: Failed to read line");
        let userInstructCache: Vec<&str> = input.split(" ").collect();
    }
}
