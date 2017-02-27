#![allow(unused_variables)]
use parser;
use parser::{Command, Redirect};
use builtins::Builtin;
use process::execute::{run, first_pipe, execute_pipe, final_pipe, redirect_out};
use std::env;
use std::collections::HashMap;

pub enum ReturnValue {
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

pub fn interpet_line(line: String, builtins: &HashMap<String, Builtin>) -> ReturnValue {
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
    //println!("{:?}", parse_tree);
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
    exec_command(&current)
}

fn exec_command(current: &Command) -> ReturnValue {
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