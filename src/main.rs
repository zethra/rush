#![allow(unused_must_use)]

extern crate rush;
extern crate rustyline;
extern crate libc;
extern crate nix;

use rush::builtins;
use rush::prompt::Prompt;
use rush::interpeter::*;
use rustyline::error::ReadlineError;
use rustyline::{Config, CompletionType, Editor};
use rustyline::completion::FilenameCompleter;
use std::env::home_dir;
use std::process;
use std::env;
use std::io::{BufReader, BufRead};
use std::fs::File;


fn main() {
    let mut exit_status = 0;
    #[cfg(unix)]    {
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
            Ok(_) => {}
            Err(_) => println!("Couldn't set pgid"),
        };
        match nix::unistd::tcsetpgrp(0, pid) {
            Ok(_) => {}
            Err(_) => println!("Couldn't set process to foreground"),
        }
    }

    let builtins = builtins::get_builtins();

    let mut home_config = home_dir().expect("No Home directory");
    home_config.push(".rushrc");
    let f = match File::open(&home_config) {
        Ok(f) => f,
        Err(_) => {
            println!("Couldn't open file .rushrc");
            return;
        }
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        match interpet_line(l, &builtins) {
            ReturnValue::True => {}
            ReturnValue::False => {}
            ReturnValue::Exit(v) => {
                exit_status = v;
                break;
            }
        }
    }

    let mut cmd_args = env::args().skip(1);
    let file_name = cmd_args.next();
    if file_name.is_some() {
        let file_name = file_name.unwrap();
        let f = match File::open(&file_name) {
            Ok(f) => f,
            Err(_) => {
                println!("Couldn't open file {}", file_name);
                return;
            }
        };
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = line.unwrap();
            match interpet_line(l, &builtins) {
                ReturnValue::True => {}
                ReturnValue::False => {}
                ReturnValue::Exit(v) => {
                    exit_status = v;
                    break;
                }
            }
        }
        process::exit(exit_status);
    }

    let mut home_config = home_dir().expect("No Home directory");
    home_config.push(".rush_history");
    let history =
        home_config.as_path().to_str().expect("Should have a home directory to turn into a str");

    // Set up buffer to read inputs and History Buffer
    let input_config = Config::builder().completion_type(CompletionType::List).build();
    let mut input_buffer = Editor::with_config(input_config);
    input_buffer.set_completer(Some(FilenameCompleter::new()));
    if let Err(_) = input_buffer.load_history(history) {
        println!("No previous history.");
    }
    let mut prompt = Prompt::new();

    // Loop to recieve and execute commands
    loop {
        prompt.print();
        let line = input_buffer.readline(&prompt.get_user_p());
        match line {
            Ok(line) => {
                input_buffer.add_history_entry(line.as_ref());
                match interpet_line(line, &builtins) {
                    ReturnValue::True => {}
                    ReturnValue::False => {}
                    ReturnValue::Exit(v) => {
                        exit_status = v;
                        break;
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                print!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                exit_status = 1;
                break;
            }
        }
    }
    input_buffer.save_history(history).unwrap();
    process::exit(exit_status);
}