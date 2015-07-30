//#![feature(plugin)]
//#![plugin(clippy)]
#[macro_use] extern crate rusty;
use rusty::utils::*;
use rusty::core::execute::interpret;
use rusty::core::buffer_in::*;
use rusty::core::prompt::Prompt;
use rusty::core::config::{check_alias,set_env_var};
use std::io::{stdin,Write,stdout};

fn main() {
    //Sets environment variables written in config file
    set_env_var();

    //Necessary to update as default prompt is not what we want
    //They were merely initialization values
    let mut prompt = Prompt::new();
    prompt.update_cwd();
    prompt.update_prompt();
    print!("{} ", prompt.get_user_p());
    stdout().flush().ok().expect("Failed to put prompt on line");

    //Set up buffer to read inputs
    let mut input_buffer = InputBuffer::new(); //Make sure to get rid of _ later
    input_buffer.readline();
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
                let alias = check_alias(command_split.clone());
                if !alias.is_some() {
                    let output = interpret(command_split);
                    if !output.is_empty() {
                        println!("{}",output.trim());
                    }
                } else {
                    //Removes alias from the non cloned version like check_alias() does
                    command_split.remove(0);
                    let alias_unwrapped = alias.unwrap().to_owned();
                    let mut vec: Vec<&str> = alias_unwrapped.trim().split(' ').collect();
                    for i in command_split {
                        vec.push(i);
                    }
                    let output =  interpret(vec);
                    if !output.is_empty() {
                        println!("{}",output.trim());
                    }
                }
            }
        }
        //Things that must always run in order to work.
        //Input Buffer Clean Up and Update

        //History Clean Up and Update

        //Prompt Printing
        print!("{} ", prompt.get_user_p());
        stdout().flush().ok().expect("Failed to put prompt on line");
    }

}
