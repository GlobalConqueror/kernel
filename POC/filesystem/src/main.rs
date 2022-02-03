use std::io;
use std::io::*;
use std::env;

#[derive(Debug)]
pub struct File {
    name: String
}

#[allow(non_snake_case)]
fn main() {
    /*Variables*/
    let run: bool = true;
    let mut files = vec![File { name: "template.txt".parse().unwrap()}];
    /*On Start-Up*/
    println!("  DEMO SHELL");
    /*Main program*/
    let mut args: Vec<String> = env::args().collect();
    /*
    - Implement some way of removing the exe call
    - If done, change the if, else if statements to evaluate args[0] instead of args[1]
    */
        println!("{:#?}", args);
    /*Input evaluation - Does not work*/
    if args[1] == "touch" {
        let newFile = File { name: args[1].parse().unwrap(),};
        files.push(newFile);
    }
    else if args[1] == "ls" || args[0] == "dir" {
        for i in 0..files.len()  {
            println!("{:#?}", files[i].name);
        }
    }
    else {
        eprintln!("Invalid command");
    }
}
