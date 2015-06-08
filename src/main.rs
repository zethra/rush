#![allow(unused_imports)]
extern crate rusty;

use rusty::utils::*;
use rusty::core::*;
use std::path::Path;

fn main() {
    //Initialization occurs here 
    execute::execute("ls -al /home/michael");
    execute::execute("          ls".trim());
    //Loop to recieve and execute commands
    loop{
        
        let mut command = "exit";
        
        //Determines what to do with user input
        //problem _ might need to be used to execute command
        match command {
            "exit" => break,
            _ => println!("Command: {} not found", command),
        }
    }

}
