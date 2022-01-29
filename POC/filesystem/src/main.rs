use std::io;
use std::io::*;
#[allow(non_snake_case)]

#[derive(Debug)]
pub struct File {
    name: String
}

fn main() {
    /*Variables*/
    let run: bool = true;
    let mut input = String::new();
    let mut files = vec![File { name: "template.txt".parse().unwrap()}];
    /*On Start-Up*/
    println!("  DEMO SHELL");
    /*Main program*/
    while run {
        print!("> ");
        io::stdout().flush().unwrap();
        stdin().read_line(&mut input).ok().expect("Error: Failed to read line");
        let userInstructCache: Vec<&str> = input.split(" ").collect();
        if userInstructCache[0] == "touch" {
            let newFile = File { name: userInstructCache[1].parse().unwrap(),};
            files.push(newFile);
            continue;
        }
        else if userInstructCache[0] == "ls" || userInstructCache[0] == "dir" {
            for i in 0..files.len()  {
                println!("{:#?}", files[i].name);
            }
        }
    }
}
