#![feature(plugin)]
#![plugin(clippy)]

#![cfg(not(test))]
#[macro_use] extern crate rush;
use rush::utils::*;
use rush::process::execute::interpret;
use rush::input::*;
use rush::history::*;
use rush::prompt::Prompt;
use rush::config::{check_alias,set_env_var};
use rush::keybinding::*;
use std::thread;

fn main() {
    //Sets environment variables written in config file
    set_env_var();

    //Necessary to update as default prompt is not what we want
    //They were merely initialization values
    let prompt_spawn = thread::spawn(move || {
        let thread_prompt = Prompt::new();
        thread_prompt.print();
        thread_prompt
    });

    let input_spawn = thread::spawn(move || {
        InputBuffer::new()
    });

    let history_spawn = thread::spawn(move || {
        HistoryBuffer::new()
    });

    //Set up buffer to read inputs and History Buffer
    let mut input_buffer = input_spawn.join()
        .ok().expect("No InputBuffer made");
    let mut history = history_spawn.join()
        .ok().expect("No HistoryBuffer made");
    let mut prompt = prompt_spawn.join()
        .ok().expect("No prompt made");
    //Loop to recieve and execute commands
    loop{
        input_buffer.readline(&mut history);

        let mut command_split: Vec<&str> = input_buffer.output();

        match *command_split.get(0)
            .expect("Called unwrap on an empty buffer") {

            "cd" => {
                command_split.remove(0);
                cd::change_directory(command_split);
                prompt.update_cwd();
                prompt.update_prompt();
            },

            "clear" => {
                let output = interpret(command_split.clone());
                print!("{}", output);
                prompt.print();
                continue;
            },

            ""  => {
                prompt.print();
                continue;
            },

            "exit" => break,
            _ => {
                let alias = check_alias(command_split.clone());
                if !alias.is_some() {
                    let output = interpret(command_split);
                    if !output.is_empty() {
                        println!("{}",output.trim());
                    }
                } else {
                    //Removes alias from the non cloned
                    //version like check_alias() does
                    command_split.remove(0);
                    let alias_unwrapped = alias
                        .expect("Should have returned an unwrappable value")
                        .to_owned();
                    let mut vec: Vec<&str> = alias_unwrapped
                        .trim().split(' ').collect();
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
        //Updates the prompt for the next line
        prompt.print();
    }

}
