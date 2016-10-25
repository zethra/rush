#![allow(unused_imports)] //Here until interpret is complete
extern crate libc;
extern crate posix;

use std::process::*;
use process::logic::*;
use process::stdproc::*;
use process::pipe::*;
use process::ops::*;
use process::pq::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
#[cfg(unix)]
use std::os::unix::process::CommandExt;

///Interpret
///Given an input command, interpret parses and determines what and how
///to execute it and returns output or error output
pub fn interpret(command: String) -> bool {
//    let mut op_queues = Opqueue::new();
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

    let mut args: Vec<&str> = Vec::new();
    let mut start_index = 0;
    let mut in_quotes = false;
    for (i, c) in parsed_command.chars().enumerate() {
        if c == ' ' && !in_quotes && start_index < i {
            args.push(&parsed_command[start_index..i]);
            start_index = i + 1;
        } else if c == '\u{1E}' && !in_quotes && !escape {
            in_quotes = true;
            start_index = i + 1;
        } else if c == '\u{1E}' && in_quotes && !escape && start_index < i {
            args.push(&parsed_command[start_index..i]);
            start_index = i + 1;
            in_quotes = false;
        } else if c == ' ' && !in_quotes && start_index == i {
            start_index = i + 1;
        }
    }
    if start_index < parsed_command.len() {
        args.push(&parsed_command[start_index..parsed_command.len()]);
    }

    //Split order:
    //Split by parallel +=+
    //Split by or ||
    //Split by pipe |
    //Split by and &&
    //Split by (To be expanded)

    let mut redirects = false;
    let mut pipes = false;
    for i in args.clone() {
        if i.contains('>') {
            redirects = true;
        }
        if i.contains('|') && !i.contains("||") {
            pipes = true;
        }
    }

    if pipes && redirects {
        piped_redirect(args)
    } else if pipes {
        piped(args)
    } else if redirects {
        redirect(args)
    } else {
        run(args)
    }
}

///Run
///Runs commands passed to it and returns the output
pub fn run(command: Vec<&str>) -> bool {
    let args = command.as_slice();
    let spid = unsafe {
        libc::getpid()
    };
    if args.len() > 1 {
        match Command::new(&args[0])
            .args(&args[1..])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .before_exec(|| {
                unsafe {
                    let pid = libc::getpid();
                    libc::setpgid(pid, pid);
                }
                Result::Ok(())
            })
            .spawn() {
            Ok(mut cmd) => {
                match cmd.wait() {
                    Ok(status) => {
                        status.success()
                    },
                    Err(_) => {
                        println!("failed to wait for child");
                        false
                    },
                }
            },
            Err(_) => {
                println!("Failed to execute");
                false
            },
        }
    } else if args.len() == 1 {
        match Command::new(&args[0])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .before_exec(|| {
                unsafe {
                    let pid = libc::getpid();
                    libc::setpgid(pid, pid);
                    posix::unistd::tcsetpgrp(spid, pid);
                }
                Result::Ok(())
            })
            .spawn() {
            Ok(mut child) => {
                match child.wait() {
                    Ok(status) => {
                        status.success()
                    },
                    Err(_) => {
                        println!("failed to wait for child");
                        false
                    },
                }
            },
            Err(_) => {
                println!("Failed to execute");
                false
            },
        }
    } else {
        match Command::new("")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .before_exec(|| {
                unsafe {
                    let pid = libc::getpid();
                    libc::setpgid(pid, pid);
                }
                Result::Ok(())
            })
            .spawn() {
            Ok(mut cmd) => {
                match cmd.wait() {
                    Ok(status) => {
                        status.success()
                    },
                    Err(_) => {
                        println!("failed to wait for child");
                        false
                    },
                }
            },
            Err(_) => {
                println!("Failed to execute");
                false
            },
        }
    }
}

pub fn redirect(command: Vec<&str>) -> bool {
    let mut args = command;
    let mut file_path = "".to_owned();
    for i in 0..args.len() {
        if args[i].contains('>') {
            file_path.push_str(&args[i + 1..args.len()].to_vec().join(""));
            args.truncate(i);
            break;
        }
    }
    let args = args.as_slice();
    let output = if args.len() > 1 {
        Command::new(&args[0])
            .args(&args[1..])
            .output()
            .ok()
    } else if args.len() == 1 {
        Command::new(&args[0])
            .output()
            .ok()
    } else {
        Command::new("")
            .output()
            .ok()
    };
    let str_out = if output.is_some() {
        let temp = output.expect("Output has been checked");
        if temp.stdout.is_empty() {
            String::from_utf8(temp.stderr)
                .expect("Should have translated to string easily")
        } else {
            String::from_utf8(temp.stdout)
                .expect("Should have translated to string easily")
        }
    } else {
        "".to_owned()
    };
    let path = Path::new(&file_path);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(str_out.as_bytes()) {
        panic!("couldn't write to {}: {}", display, why.description());
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
}

