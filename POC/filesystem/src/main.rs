use std::io;
use std::io::*;


#[derive(Debug)]
pub struct File {
    name: String
}

fn trim(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

#[allow(non_snake_case)]
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
        trim(&mut input);
        let mut userInstructCache: Vec<&str> = input.split(" ").collect();
        for _i in 0..userInstructCache.len() {
            println!("{:#?}", userInstructCache);
        }
        /*Input evaluation - Does not work*/
        if userInstructCache[0] == "touch" {
            let newFile = File { name: userInstructCache[1].parse().unwrap(),};
            files.push(newFile);
            continue;
        }
        else if userInstructCache[0] == "ls" || userInstructCache[0] == "dir" {
            for i in 0..files.len()  {
                println!("{:#?}", files[i].name);
            }
            continue;
        }
        else {
            eprintln!("Invalid command");
            userInstructCache.drain(..);
        }
    }
}
