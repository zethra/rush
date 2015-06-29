//#![feature(plugin)]
//#![plugin(clippy)]
extern crate rusty;
use rusty::utils::*;
use rusty::core::*;
use rusty::core::prompt::Prompt;
use std::io::{stdin,Write,stdout};

fn main() {
    //Initialization occurs here
    let prompt = Prompt::new();    
    
    print!("{} ", prompt.get_user_p());
    stdout().flush().ok().expect("Failed to put prompt on line");
    
    //Loop to recieve and execute commands
    loop{
        let mut command = String::new();
        stdin().read_line(&mut command)
            .ok()
            .expect("Failure to read input");
        
        //Super ugly if you know what types are being passed around
        //below here.It works but I will probably want to clean it up at 
        //some point

        let mut command_split: Vec<&str> = command.trim().split(' ').collect(); 
        match command_split.get(0) {
            Some(&"cd") => {
                command_split.remove(0); 
                cd::change_directory(command_split);
            }
            Some(&"cat") => {
                
            }
            Some(&"")  => continue,
            Some(&"exit") => break,
            _ => {
                let output = execute::interpret(command_split);
                if !output.is_empty() {
                    println!("{}",output.trim());
                }
            }
        }
        
        print!("{} ", prompt.get_user_p());
        stdout().flush().ok().expect("Failed to put prompt on line");
    }

}
