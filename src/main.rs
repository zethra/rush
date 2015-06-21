#![feature(plugin)]
#![plugin(clippy)]
#![allow(unused_imports)]
extern crate rusty;
use rusty::utils::*;
use rusty::core::*;
use std::path::Path;
use std::io::stdin;
use std::env;

fn main() {
    //Initialization occurs here 
    
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
    }

}
