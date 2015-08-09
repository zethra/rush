#![feature(plugin)]
//#![plugin(clippy)]

#![cfg(not(test))]
#[macro_use] extern crate rush;
use rush::utils::*;
use rush::core::execute::interpret;
use rush::core::buffer_in::*;
use rush::core::history::*;
use rush::core::prompt::Prompt;
use rush::core::config::{check_alias,set_env_var};
use rush::core::keybinding::*;

fn main() {
    //Sets environment variables written in config file
    set_env_var();

    //Necessary to update as default prompt is not what we want
    //They were merely initialization values
    let mut prompt = Prompt::new();
    prompt.update_cwd();
    prompt.update_prompt();
    prompt.print();

    //Set up buffer to read inputs and History Buffer
    let mut input_buffer = InputBuffer::new();
    let mut history = HistoryBuffer::new();
    //Loop to recieve and execute commands
    loop{
        let key = input_buffer.readline();
        history.store(input_buffer.line.clone());

        match key {
            Key::Up => {}, //Go up in History
            Key::Down => {}, //Go down in History
            Key::Null => {},
            _ => {},
        }

        let mut command_split: Vec<&str> = input_buffer.output();

        match command_split.get(0).unwrap() {

            &"cd" => {
                command_split.remove(0);
                cd::change_directory(command_split);
                prompt.update_cwd();
                prompt.update_prompt();
            }

            &"clear" => {
                let output = interpret(command_split.clone());
                print!("{}", output);
                prompt.print();
                continue;
            }

            &""  => {
                prompt.print();
                continue;
            }

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

        prompt.print();
    }

}
