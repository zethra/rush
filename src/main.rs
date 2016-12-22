#![feature(plugin)]
//#![plugin(clippy)]
#![allow(unused_must_use)]

#![cfg(not(test))]

#[macro_use] extern crate rush;
extern crate rustyline;
extern crate libc;
extern crate nix;

use rush::builtins::*;
use rush::process::execute::interpret;
use rush::prompt::Prompt;
use rush::config::{check_alias, set_env_var};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env::home_dir;

fn main() {
    #[cfg(unix)] {
        while nix::unistd::tcgetpgrp(0).unwrap() != nix::unistd::getpgrp() {
            nix::sys::signal::kill(nix::unistd::getpgrp(), nix::sys::signal::Signal::SIGTTIN);
        }
        unsafe {
            libc::signal(libc::SIGINT, libc::SIG_IGN);
            libc::signal(libc::SIGQUIT, libc::SIG_IGN);
            libc::signal(libc::SIGTSTP, libc::SIG_IGN);
            libc::signal(libc::SIGTTIN, libc::SIG_IGN);
            libc::signal(libc::SIGTTOU, libc::SIG_IGN);
        }
        let pid = nix::unistd::getpid();
        match nix::unistd::setpgid(pid, pid) {
            Ok(_) => {},
            Err(_) => println!("Couldn't set pgid"),
        };
        // Doesn't seem necessary
//        match nix::unistd::setsid() {
//            Ok(_) => {},
//            Err(_) => println!("Couldn't set sid"),
//        }
        match nix::unistd::tcsetpgrp(0, pid) {
            Ok(_) => {},
            Err(_) => println!("Couldn't set process to foreground"),
        }
    }

    //Sets environment variables written in config file
    set_env_var();

    let mut home_config = home_dir().expect("No Home directory");
    home_config.push(".rush_history");
    let history = home_config.as_path().to_str().expect("Should have a home directory to turn into a str");

    //Set up buffer to read inputs and History Buffer
    let mut input_buffer = Editor::<()>::new();
    if let Err(_) = input_buffer.load_history(history) {
        println!("No previous history.");
    }
    let mut prompt = Prompt::new();

    //Loop to recieve and execute commands
    loop {
        prompt.print();
        let line = input_buffer.readline(&prompt.get_user_p());
        match line {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                let command = line.trim().to_string();
                input_buffer.add_history_entry(&line);

                //This is hackish and a stop gap for now. The important part is that
                //a string is always being passed to interpret. Once interpret has
                //been finished Main needs to be cleaned up more so that it can
                //just use strings here
                if command.starts_with("cd") {
                    cd::change_directory(command.trim_left_matches("cd").to_owned());
                } else if command.starts_with("clear") {
                    let output = interpret(command);
                    print!("{}", output);
                    continue;
                } else if command.is_empty() {
                    continue;
                } else if command.starts_with("exit") {
                    break;
                } else {
                    let alias = check_alias(command.clone());
                    if alias.is_none() {
                        interpret(command);
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
                        interpret(vec);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                print!("^C");
            },
            Err(ReadlineError::Eof) => {
                //                println!("CTRL-D");
                //                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    input_buffer.save_history(history).unwrap();
}
