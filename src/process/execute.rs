#![allow(unused_imports)] //Here until interpret is complete
extern crate libc;
extern crate nix;

use std::process::*;
use process::stdproc::*;
use process::ops::*;
use process::pq::*;
#[cfg(unix)]
use process::unix::execute::*;
#[cfg(unix)]
use process::unix::pipe::*;
#[cfg(windows)]
use process::windows::execute::*;
#[cfg(windows)]
use process::windows::pipe::*;

///Interpret
///Given an input command, interpret parses and determines what and how
///to execute it and returns output or error output
pub fn interpret(command: String) -> bool {
    let mut op_queue = Opqueue::new();
    //    let mut proc_queue = Procqueue::new();

    let mut parsed_command = "".to_string();
    let mut escape = false;
    for c in command.chars() {
        if c == '\\' {
            escape = true;
        } else if escape {
            escape = false;
            parsed_command.push(match c {
                'a' => '\u{07}',
                'b' => '\u{08}',
                'f' => '\u{0C}',
                'n' => '\u{0A}',
                'r' => '\u{0D}',
                't' => '\u{09}',
                'v' => '\u{0B}',
                '\\' => '\u{5C}',
                '\'' => '\u{27}',
                '"' => '\u{22}',
                _ => continue,
            })
        } else if c == '"' {
            parsed_command.push('\u{1E}');
        } else if c as u8 >= 32 && c as u8 <= 126 {
            parsed_command.push(c);
        }
    }

    let mut commands: Vec<Operation> = Vec::new();
    let mut args: Vec<String> = Vec::new();
    let mut start_index = 0;
    let mut in_quotes = false;
    let mut next_piped = false;
    let mut next_redirect_out = false;
    for (i, c) in parsed_command.chars().enumerate() {
        if c == ' ' && !in_quotes && start_index < i {
            match parsed_command[start_index..i].as_ref() {
                "&&" => {
                    if next_piped && next_redirect_out {
                        commands.push(Operation::PipeRedirectOut { val: args });
                    } else if next_piped {
                        commands.push(Operation::Pipe { val: args });
                    } else if next_redirect_out {
                        commands.push(Operation::RedirectOut { val: args });
                    } else {
                        commands.push(Operation::Command { val: args });
                    }
                    next_piped = false;
                    next_redirect_out = false;
                    commands.push(Operation::And);
                    args = Vec::new();
                },
                "||" => {
                    if next_piped && next_redirect_out {
                        commands.push(Operation::PipeRedirectOut { val: args });
                    } else if next_piped {
                        commands.push(Operation::Pipe { val: args });
                    } else if next_redirect_out {
                        commands.push(Operation::RedirectOut { val: args });
                    } else {
                        commands.push(Operation::Command { val: args });
                    }
                    next_piped = false;
                    next_redirect_out = false;
                    commands.push(Operation::Or);
                    args = Vec::new();
                },
                "|" => {
                    next_piped = true;
                    args.push("|".to_string());
                },
                ">" => {
                    next_redirect_out = true;
                    args.push(">".to_string());
                },
                "&" => {
                    if next_piped && next_redirect_out {
                        commands.push(Operation::PipeRedirectOut { val: args });
                    } else if next_piped {
                        commands.push(Operation::Pipe { val: args });
                    } else if next_redirect_out {
                        commands.push(Operation::RedirectOut { val: args });
                    } else {
                        commands.push(Operation::Command { val: args });
                    }
                    next_piped = false;
                    next_redirect_out = false;
                    commands.insert(0, Operation::Detached);
                    op_queue.push(commands.clone());
                    commands = Vec::new();
                    args = Vec::new();
                },
                _ => {
                    args.push(parsed_command[start_index..i].to_string());
                }
            }
            start_index = i + 1;
        } else if c == '\u{1E}' && !in_quotes && !escape {
            in_quotes = true;
            start_index = i + 1;
        } else if c == '\u{1E}' && in_quotes && !escape && start_index < i {
            args.push(parsed_command[start_index..i].to_string());
            start_index = i + 1;
            in_quotes = false;
        } else if c == ' ' && !in_quotes && start_index == i {
            start_index = i + 1;
        }
    }
    if start_index < parsed_command.len() {
        match parsed_command[start_index..parsed_command.len()].as_ref() {
            "&&" => {
                if next_piped && next_redirect_out {
                    commands.push(Operation::PipeRedirectOut { val: args });
                } else if next_piped {
                    commands.push(Operation::Pipe { val: args });
                } else if next_redirect_out {
                    commands.push(Operation::RedirectOut { val: args });
                } else {
                    commands.push(Operation::Command { val: args });
                }
                next_piped = false;
                next_redirect_out = false;
                commands.push(Operation::And);
                args = Vec::new();
            },
            "||" => {
                if next_piped && next_redirect_out {
                    commands.push(Operation::PipeRedirectOut { val: args });
                } else if next_piped {
                    commands.push(Operation::Pipe { val: args });
                } else if next_redirect_out {
                    commands.push(Operation::RedirectOut { val: args });
                } else {
                    commands.push(Operation::Command { val: args });
                }
                next_piped = false;
                next_redirect_out = false;
                commands.push(Operation::Or);
                args = Vec::new();
            },
            "|" => {
                next_piped = true;
                args.push("|".to_string());
            },
            ">" => {
                next_redirect_out = true;
                args.push(">".to_string());
            },
            "&" => {
                if next_piped && next_redirect_out {
                    commands.push(Operation::PipeRedirectOut { val: args });
                } else if next_piped {
                    commands.push(Operation::Pipe { val: args });
                } else if next_redirect_out {
                    commands.push(Operation::RedirectOut { val: args });
                } else {
                    commands.push(Operation::Command { val: args });
                }
                next_piped = false;
                next_redirect_out = false;
                commands.insert(0, Operation::Detached);
                op_queue.push(commands.clone());
                commands = Vec::new();
                args = Vec::new();
            },
            _ => args.push(parsed_command[start_index..parsed_command.len()].to_string())
        }
    }
    if args.len() > 0 {
        if next_piped && next_redirect_out {
            commands.push(Operation::PipeRedirectOut { val: args });
        } else if next_piped {
            commands.push(Operation::Pipe { val: args });
        } else if next_redirect_out {
            commands.push(Operation::RedirectOut { val: args });
        } else {
            commands.push(Operation::Command { val: args });
        }
    }
    op_queue.push(commands);
//    println!("{:?}", op_queue);

    loop {
        match op_queue.pop() {
            Some(args) => {
                let mut iter = args.iter().enumerate();
                let mut last_return = true;
                let mut next_op: Operation = Operation::And;
                let mut detached = false;
                loop {
                    match iter.next() {
                        Some((i, arg)) => {
                            if i == 0 {
                                match arg {
                                    &Operation::Command { ref val } => {
                                        last_return = run(val.clone());
                                    },
                                    &Operation::Pipe { ref val } => {
                                        last_return = piped(val.clone());
                                    },
                                    &Operation::RedirectOut { ref val } => {
                                        last_return = redirect_out(val.clone());
                                    },
                                    &Operation::PipeRedirectOut { ref val } => {
                                        last_return = piped_redirect_out(val.clone());
                                    },
                                    &Operation::Detached => {
                                        detached = true;
                                    },
                                    _ => println!("Parse Error 1"),
                                }
                            } else if detached && i == 1 {
                                match arg {
                                    &Operation::Command { ref val } => {
                                        last_return = run_detached(val.clone());
                                    },
                                    &Operation::Pipe { ref val } => {
                                        last_return = piped(val.clone());
                                    },
                                    &Operation::RedirectOut { ref val } => {
                                        last_return = redirect_out(val.clone());
                                    },
                                    &Operation::PipeRedirectOut { ref val } => {
                                        last_return = piped_redirect_out(val.clone());
                                    },
                                    _ => println!("Parse Error 6"),
                                }
                            } else {
                                match arg {
                                    &Operation::And => next_op = Operation::And,
                                    &Operation::Or => next_op = Operation::Or,
                                    &Operation::Pipe { ref val } => {
                                        match next_op {
                                            Operation::And => {
                                                if last_return {
                                                    last_return = piped(val.clone());
                                                } else {
                                                    last_return = false;
                                                }
                                            },
                                            Operation::Or => {
                                                if last_return == false {
                                                    last_return = piped(val.clone());
                                                }
                                            },
                                            _ => println!("Parse Error 2"),
                                        }
                                    },
                                    &Operation::Command { ref val } => {
                                        match next_op {
                                            Operation::And => {
                                                if last_return {
                                                    last_return = run(val.clone());
                                                } else {
                                                    last_return = false;
                                                }
                                            },
                                            Operation::Or => {
                                                if last_return == false {
                                                    last_return = run(val.clone());
                                                }
                                            },
                                            _ => println!("Parse Error 3"),
                                        }
                                    },
                                    &Operation::RedirectOut { ref val } => {
                                        match next_op {
                                            Operation::And => {
                                                if last_return {
                                                    last_return = redirect_out(val.clone());
                                                } else {
                                                    last_return = false;
                                                }
                                            },
                                            Operation::Or => {
                                                if last_return == false {
                                                    last_return = redirect_out(val.clone());
                                                }
                                            },
                                            _ => println!("Parse Error 4"),
                                        }
                                    },
                                    &Operation::PipeRedirectOut { ref val } => {
                                        match next_op {
                                            Operation::And => {
                                                if last_return {
                                                    last_return = redirect_out(val.clone());
                                                } else {
                                                    last_return = false;
                                                }
                                            },
                                            Operation::Or => {
                                                if last_return == false {
                                                    last_return = redirect_out(val.clone());
                                                }
                                            },
                                            _ => println!("Parse Error 5"),
                                        }
                                    },
                                    &Operation::Detached => println!("Parse Error 7"),
                                }
                            }
                        },
                        None => break,
                    }
                }
            },
            None => break,
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;
}

