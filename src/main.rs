#![feature(plugin)]
#![plugin(clippy)]

#![cfg(not(test))]

#[macro_use] extern crate rush;
extern crate rustyline;

use rush::utils::*;
use rush::process::execute::interpret;
use rush::prompt::Prompt;
use rush::config::{check_alias, set_env_var};
use std::thread;
use rustyline::error::ReadlineError;
use rustyline::Editor;

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

    //Set up buffer to read inputs and History Buffer
    let mut input_buffer = Editor::<()>::new();
    //    if let Err(_) = rl.load_history("history.txt") {
    //        println!("No previous history.");
    //    }
    let mut prompt = prompt_spawn.join().expect("No prompt made");
    //Loop to recieve and execute commands
    loop {
        let line = input_buffer.readline(&prompt.get_user_p());
        match line {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                input_buffer.add_history_entry(&line);
                let command = line;
                //This is hackish and a stop gap for now. The important part is that
                //a string is always being passed to interpret. Once interpret has
                //been finished Main needs to be cleaned up more so that it can
                //just use strings here
                if command.starts_with("cd") {
                    cd::change_directory(command.trim_left_matches("cd").to_owned());
                    prompt.update_cwd();
                    prompt.update_prompt();
                } else if command.starts_with("clear") {
                    let output = interpret(command);
                    print!("{}", output);
                    prompt.print();
                    continue;
                } else if command.is_empty() {
                    prompt.print();
                    continue;
                } else if command.starts_with("exit") {
                    break;
                } else if command.starts_with("pwd") {
                    println!("{}", prompt.get_cwn_abs());
                    prompt.print();
                    continue;
                } else {
                    let alias = check_alias(command.clone());
                    if alias.is_none() {
                        let output = interpret(command);
                        if !output.is_empty() {
                            println!("{}", output.trim());
                        }
                    } else {
                        //Removes alias from the non cloned
                        //version like check_alias() does
                        let mut vec = alias
                            .expect("Should have returned an unwrappable value")
                            .to_owned();

                        //Removes alias and pushes the rest of the split onto
                        //the string
                        for (i, j) in command.split_whitespace()
                            .collect::<Vec<&str>>().iter().enumerate() {
                            if i != 0 {
                                vec.push_str(j);
                            }
                        }

                        //Temporary as this will get resplit in interpret
                        let output = interpret(vec);
                        if !output.is_empty() {
                            println!("{}", output.trim());
                        }
                    }
                }
                //Updates the prompt for the next line
                prompt.print();
            },
            Err(ReadlineError::Interrupted) => {
                print!("^C");
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    //    input_buffer.save_history("history.txt").unwrap();
}
