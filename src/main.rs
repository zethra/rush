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
use rush::prompt::Prompt;
use rush::config::{check_alias, set_env_var};
use rush::parser;
use rush::parser::{Statement, Command, Redirect};
use rush::builtins::Builtin;
use rush::process::execute::{run, first_pipe, execute_pipe, final_pipe, redirect_out};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env::home_dir;
use std::process;
use std::env;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;


enum ReturnValue {
    True,
    False,
    Exit(i32),
}

trait ToReturnVal {
    fn to_return_val(self) -> ReturnValue;
}

impl ToReturnVal for bool {
    fn to_return_val(self) -> ReturnValue {
        if self {
            ReturnValue::True
        } else {
            ReturnValue::False
        }
    }
}

fn interpet_line(line: String, builtins: &HashMap<String, Builtin>) -> ReturnValue {
    if line.is_empty() {
        return ReturnValue::True;
    }
    let command = line.to_string();

    let parse_tree = match parser::script(&command) {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            return ReturnValue::False;
        }
    };
    if parse_tree.is_none() {
        return ReturnValue::True;
    }
    let parse_tree = parse_tree.unwrap();
    // println!("{:?}", parse_tree);
    let mut current = parse_tree.0.statement;
    replace_vars(&mut current);
    if current.name == "exit".to_string() {
        if current.args.len() > 0 {
            match current.args[0].parse::<i32>() {
                Ok(e) => return ReturnValue::Exit(e),
                Err(_) => {
                    println!("exit requires numberic value");
                    return ReturnValue::Exit(0);
                }
            }
        }
        return ReturnValue::Exit(0);
    }
    if builtins.contains_key(&current.name) {
        match builtins.get(&current.name) {
            Some(f) => f(&current.args),
            None => {
                println!("Builtin Error");
                return ReturnValue::False;
            }
        };
        return ReturnValue::True;
    }
    if current.pipe.is_some() {
        let child_result = first_pipe(&current.name, &current.args, &current.vars);
        let mut child = child_result.expect("Failed to unwrap an Result");
        loop {
            let mut next = current.pipe.unwrap();
            replace_vars(&mut next);
            if next.pipe.is_some() {
                let child_result = execute_pipe(&next.name, &next.args, &current.vars, child);
                child = child_result.expect("Failed to unwrap an Result");
                current = *next;
            } else {
                return final_pipe(&next.name, &next.args, &current.vars, child).to_return_val();
            }
        }
    } else if current.redirect.is_some() {
        let redirect = current.redirect.unwrap();
        match redirect {
            Redirect::Fd(fd, op, file_name) => {
                match op.as_str() {
                    ">" => {
                        return redirect_out(&current.name,
                                            &current.args,
                                            &current.vars,
                                            &file_name)
                            .to_return_val();
                    }
                    _ => {
                        println!("That redirect operation is not yet supported");
                        return ReturnValue::False;
                    }
                };
            }
            Redirect::DuplicateFd(_, _, _) => {
                return ReturnValue::False;
            }
            Redirect::MoveFd(_, _, _) => {
                return ReturnValue::False;
            }
        }
    } else {
        return run(&current.name, &current.args, &current.vars).to_return_val();
    }
}


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
        return;
    }

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
                input_buffer.add_history_entry(&line);
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