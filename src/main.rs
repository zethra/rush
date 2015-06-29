//#![feature(plugin)]
//#![plugin(clippy)]
extern crate rusty;
use rusty::utils::*;
use rusty::core::*;
use rusty::core::prompt::Prompt;
use std::io::{stdin,Write,stdout};

fn main() {
    //Initialization occurs here
    //Necessary to update as default prompt is not what we want
    let mut prompt = Prompt::new();    
    prompt.update_cwd();
    prompt.update_prompt();
    
    print!("{} ", prompt.get_user_p());
    stdout().flush().ok().expect("Failed to put prompt on line");
    
    //Loop to recieve and execute commands
    loop{
        let mut command = String::new();
        stdin().read_line(&mut command)
            .ok()
            .expect("Failure to read input");
        
        let mut command_split: Vec<&str> = command.trim().split(' ').collect(); 
        match command_split.get(0).unwrap() {
            
            &"cd" => {
                command_split.remove(0); 
                cd::change_directory(command_split);
                prompt.update_cwd();
                prompt.update_prompt();
            }
            
            &"cat" => {
                
            }
            
            &""  => continue,
            
            &"exit" => break,
            
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
