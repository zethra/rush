#![feature(plugin)]
//#![plugin(clippy)]
#![feature(stmt_expr_attributes)]
#![allow(unused_must_use)]

#![cfg(not(test))]

#[macro_use]
extern crate rush;
extern crate rustyline;
extern crate libc;
extern crate nix;

use rush::builtins;
use rush::process::execute::*;
use rush::prompt::Prompt;
use rush::config::{check_alias, set_env_var};
use rush::parser;
use rush::parser::{Statement, Command, Redirect};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env::home_dir;
use std::process;
use std::env;

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

    // Sets environment variables written in config file
    set_env_var();

    let builtins = builtins::get_builtins();

    let mut home_config = home_dir().expect("No Home directory");
    home_config.push(".rush_history");
    let history =
        home_config.as_path().to_str().expect("Should have a home directory to turn into a str");

    // Set up buffer to read inputs and History Buffer
    let mut input_buffer = Editor::<()>::new();
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
                if line.is_empty() {
                    continue;
                }
                let command = line.to_string();
                input_buffer.add_history_entry(&line);
                let parse_tree = match parser::script(&command) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("{:?}", e);
                        continue;
                    }
                };
                if parse_tree.is_none() {
                    continue;
                }
                let parse_tree = parse_tree.unwrap();
                // println!("{:?}", parse_tree);
                let mut current = parse_tree.0.statement;
                replace_vars(&mut current);
                if current.name == "exit".to_string() {
                    if current.args.len() > 0 {
                        exit_status = match current.args[0].parse::<i32>() {
                            Ok(e) => e,
                            Err(_) => {
                                println!("exit requires numberic value");
                                0
                            }
                        }
                    }
                    break;
                }
                if builtins.contains_key(&current.name) {
                    match builtins.get(&current.name) {
                        Some(f) => f(&current.args),
                        None => {
                            println!("Builtin Error");
                            false
                        }
                    };
                    continue;
                }
                if current.pipe.is_some() {
                    let child_result = first_pipe(&current.name, &current.args, &current.vars);
                    let mut child = child_result.expect("Failed to unwrap an Result");
                    loop {
                        let mut next = current.pipe.unwrap();
                        replace_vars(&mut next);
                        if next.pipe.is_some() {
                            let child_result =
                                execute_pipe(&next.name, &next.args, &current.vars, child);
                            child = child_result.expect("Failed to unwrap an Result");
                            current = *next;
                        } else {
                            final_pipe(&next.name, &next.args, &current.vars, child);
                            break;
                        }
                    }
                } else if current.redirect.is_some() {
                    let redirect = current.redirect.unwrap();
                    match redirect {
                        Redirect::Fd(fd, op, file_name) => {
                            match op.as_str() {
                                ">" => {
                                    redirect_out(&current.name,
                                                 &current.args,
                                                 &current.vars,
                                                 &file_name)
                                }
                                _ => {
                                    println!("That redirect operation is not yet supported");
                                    false
                                }
                            };
                        }
                        Redirect::DuplicateFd(_, _, _) => {}
                        Redirect::MoveFd(_, _, _) => {}
                    }
                } else {
                    run(&current.name, &current.args, &current.vars);
                }

            }
            Err(ReadlineError::Interrupted) => {
                print!("^C");
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    input_buffer.save_history(history).unwrap();
    process::exit(exit_status);
}

fn replace_vars(cmd: &mut Command) {
    cmd.name = replace_var(&cmd.name, &cmd.vars);
    cmd.args = cmd.args
        .iter()
        .map(|arg| replace_var(&arg, &cmd.vars))
        .collect();
}


fn replace_var(arg: &String, vars: &Vec<(String, Option<String>)>) -> String {
    if arg.chars().next().unwrap() == '$' {
        let s = arg[1..].to_string();
        for var in vars {
            if var.0 == s {
                return match &var.1 {
                    &Some(ref v) => v.clone(),
                    &None => "".to_string(),
                };
            }
        }
        return match env::var(s) {
            Ok(v) => v,
            Err(_) => "".to_string(),
        };
    } else {
        arg.clone()
    }
}