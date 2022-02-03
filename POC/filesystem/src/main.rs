use env;

#[derive(Debug)]
pub struct File {
    name: String
}

#[allow(non_snake_case)]
fn main() {
    /*Variables*/
    let mut files = vec![File { name: "template.txt".parse().unwrap()}];
    /*On Start-Up*/
    println!("  DEMO SHELL");
    /*Main program*/
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!("{:#?}", args);
    /*Input evaluation - Does not work*/
    if args[0] == "touch" {
        let newFile = File { name: args[0].parse().unwrap(),};
        files.push(newFile);
    }
    else if args[0] == "ls" || args[0] == "dir" {
        for i in 0..files.len()  {
            println!("{:#?}", files[i].name);
        }
    }
    else {
        eprintln!("Invalid command");
    }
}
