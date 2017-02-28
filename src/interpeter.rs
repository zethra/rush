#![allow(unused_variables)]
use parser;
use parser::{Command, Redirect};
use builtins::Builtin;
use process::execute::{run, first_pipe, execute_pipe, final_pipe, redirect_out,
                       final_piped_redirect_out};
use std::env;
use std::collections::HashMap;

pub fn interpet_line(line: String, builtins: &HashMap<String, Builtin>) -> bool {
    if line.is_empty() {
        return true;
    }
    let command = line.to_string();

    let parse_tree = match parser::script(&command) {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            return false;
        }
    };
    if parse_tree.is_none() {
        return true;
    }
    let parse_tree = parse_tree.unwrap();
    println!("{:?}", parse_tree);
    let mut current = parse_tree.0.statement;
    replace_vars(&mut current);
    exec_command(current, builtins)
}

fn exec_command(current: Command, builtins: &HashMap<String, Builtin>) -> bool {
    let mut current = current;
    if builtins.contains_key(&current.name) {
        match builtins.get(&current.name) {
            Some(f) => f(&current.args),
            None => {
                println!("Builtin Error");
                return false;
            }
        };
        return true;
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
                if next.redirect.is_some() {
                    match next.redirect.unwrap() {
                        Redirect::Fd(fd, op, file_name) => {
                            match op.as_str() {
                                ">" => {
                                    return final_piped_redirect_out(&current.name,
                                                                    &current.args,
                                                                    &current.vars,
                                                                    child,
                                                                    &file_name);
                                }
                                _ => {
                                    println!("That redirect operation is not yet supported");
                                    return false;
                                }
                            };
                        }
                        Redirect::DuplicateFd(_, _, _) => {
                            return false;
                        }
                        Redirect::MoveFd(_, _, _) => {
                            return false;
                        }
                    }
                } else {
                    return final_pipe(&next.name, &next.args, &current.vars, child);
                }
            }
        }
    } else if current.redirect.is_some() {
        match current.redirect.unwrap() {
            Redirect::Fd(fd, op, file_name) => {
                match op.as_str() {
                    ">" => {
                        return redirect_out(&current.name,
                                            &current.args,
                                            &current.vars,
                                            &file_name);
                    }
                    _ => {
                        println!("That redirect operation is not yet supported");
                        return false;
                    }
                };
            }
            Redirect::DuplicateFd(_, _, _) => {
                return false;
            }
            Redirect::MoveFd(_, _, _) => {
                return false;
            }
        }
    } else {
        return run(&current.name, &current.args, &current.vars);
    }
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