#![allow(unused_must_use)]
extern crate libc;
extern crate nix;

use std::process::{Stdio, Command, Child};
use std::os::unix::io::{FromRawFd, AsRawFd};
use std::io;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::os::unix::process::CommandExt;
use std::thread;

///Split Pipes
///Given a command with pipes in it, it will split them into separate
///commands to be executed
fn split_pipes(input: Vec<String>) -> Vec<Vec<String>> {
    let input_slice = input.as_slice();
    let mut pipe_commands: Vec<Vec<String>> = Vec::new();
    let mut temp: Vec<String> = Vec::new();
    for i in input_slice {
        if i.contains('|') {
            let mut splits = i.split('|');
            temp.push(splits.next()
                .expect("Failed to split pipes").to_string());
            if temp.last()
                .expect("Called last on an empty vec") == &"" {
                temp.pop();
            }
            pipe_commands.push(temp.clone());
            temp.clear();
            temp.push(splits.next()
                .expect("Unwrapped a non existent value").to_string());
            if temp.last()
                .expect("Unwrapped an empty list") == &"" {
                temp.pop();
            }
        } else {
            temp.push(i.to_string());
        }
    }
    pipe_commands.push(temp);
    pipe_commands
}

///Piped
///The logic of piping is done here and calls the functions that execute
///the pipes and returns the result
pub fn piped(input: Vec<String>) -> bool {
    let mut split = split_pipes(input);
    let mut child_result = first_pipe(split.remove(0));
    let mut child: Child;

    child = child_result.expect("Failed to unwrap an Result");

    while split.len() > 1 {
        child_result = execute_pipe(split.remove(0), child);
        child = child_result.expect("Failed to unwrap an Result");
    }

    final_pipe(split.remove(0), child)
}

pub fn piped_detached(input: Vec<String>) -> bool {
    thread::spawn(move || {
        let mut split = split_pipes(input);
        let mut child_result = first_pipe(split.remove(0));
        let mut child: Child;

        child = child_result.expect("Failed to unwrap an Result");
        let child_pgid = child.id() as i32;
        println!("{}", child_pgid);

        while split.len() > 1 {
            child_result = execute_pipe(split.remove(0), child);
            child = child_result.expect("Failed to unwrap an Result");
        }

        final_pipe_detached(split.remove(0), child);
    });
    true
}

pub fn piped_redirect_out(input: Vec<String>) -> bool {
    let mut split = split_pipes(input);
    let mut child_result = first_pipe(split.remove(0));
    let mut child: Child;

    child = child_result.expect("Failed to unwrap an Result");

    while split.len() > 1 {
        child_result = execute_pipe(split.remove(0), child);
        child = child_result.expect("Failed to unwrap an Result");
    }

    final_piped_redirect_out(split.remove(0), child)
}

pub fn piped_redirect_out_detached(input: Vec<String>) -> bool {
    thread::spawn(move || {
        let mut split = split_pipes(input);
        let mut child_result = first_pipe(split.remove(0));
        let mut child: Child;

        child = child_result.expect("Failed to unwrap an Result");
        let child_pgid = child.id() as i32;
        println!("{}", child_pgid);

        while split.len() > 1 {
            child_result = execute_pipe(split.remove(0), child);
            child = child_result.expect("Failed to unwrap an Result");
        }

        final_piped_redirect_out_detached(split.remove(0), child);
    });
    true
}

///First Pipe
///Always executed if piping and returns the child process to be used
///for the next pipe.
fn first_pipe(command: Vec<String>) -> io::Result<Child> {
    let args = command.as_slice();
    // TODO Handle args having length 0
    let mut cmd = Command::new(&args[0]);
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }
    cmd.stdout(Stdio::piped())
        .before_exec(move || {
            let pid = nix::unistd::getpid();
            nix::unistd::setpgid(pid, pid);
            unsafe {
                libc::signal(libc::SIGINT, libc::SIG_DFL);
                libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                libc::prctl(1, libc::SIGHUP);
            }
            Result::Ok(())
        })
        .spawn()
}

///Execute Pipe
///Used if there are more than two commands with piping. Takes a Child process
///as input for the next pipe and returns a Child process.
fn execute_pipe(command: Vec<String>, child: Child) -> io::Result<Child> {
    let args = command.as_slice();
    // TODO Handle args having length 0
    let mut cmd = Command::new(&args[0]);
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }
    unsafe {
        cmd.stdout(Stdio::piped())
            .stdin(Stdio::from_raw_fd(child.stdout
                .expect("No stdout").as_raw_fd()))
            .before_exec(move || {
                let pid = nix::unistd::getpid();
                nix::unistd::setpgid(pid, pid);
                libc::signal(libc::SIGINT, libc::SIG_DFL);
                libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                libc::prctl(1, libc::SIGHUP);
                Result::Ok(())
            })
            .spawn()
    }
}

///Final Pipe
///Always executed when piping processes. Takes a child process as input
///and returns the output of piping the commands.
fn final_pipe(command: Vec<String>, child: Child) -> bool {
    let args = command.as_slice();
    if args.len() <= 0 {
        return true
    }
    let mut cmd = Command::new(&args[0]);
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }
    unsafe {
        match cmd.args(&args[1..])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::from_raw_fd(child.stdout
                .expect("No stdout for child process")
                .as_raw_fd()))
            .before_exec(move || {
                let pid = nix::unistd::getpid();
                nix::unistd::setpgid(pid, pid);
                libc::signal(libc::SIGINT, libc::SIG_DFL);
                libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                libc::prctl(1, libc::SIGHUP);
                Result::Ok(())
            })
            .spawn() {
            Ok(mut child) => {
                let child_pgid = child.id() as i32;
                nix::unistd::tcsetpgrp(0, child_pgid);
                match child.wait() {
                    Ok(status) => {
                        nix::unistd::tcsetpgrp(0, nix::unistd::getpid());
                        status.success()
                    },
                    Err(_) => {
                        nix::unistd::tcsetpgrp(0, nix::unistd::getpid());
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

fn final_pipe_detached(command: Vec<String>, child: Child) -> bool {
    let args = command.as_slice();
    if args.len() <= 0 {
        return true
    }
    let mut cmd = Command::new(&args[0]);
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }
    unsafe {
        match cmd.args(&args[1..])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::from_raw_fd(child.stdout
                .expect("No stdout for child process")
                .as_raw_fd()))
            .before_exec(move || {
                let pid = nix::unistd::getpid();
                nix::unistd::setpgid(pid, pid);
                libc::signal(libc::SIGINT, libc::SIG_DFL);
                libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                libc::prctl(1, libc::SIGHUP);
                Result::Ok(())
            })
            .spawn() {
            Ok(mut child) => {
                let child_pgid = child.id() as i32;
                println!("{}", child_pgid);
                match child.wait() {
                    Ok(status) => {
                        if status.success() {
                            println!("+ {} done", child_pgid);
                        } else {
                            match status.code() {
                                Some(c) => println!("+ {} exit {}", child_pgid, c),
                                None => println!("+ {} error", child_pgid),
                            }
                        }
                    },
                    Err(_) => {
                        println!("+ {} Failed to wait for child", child_pgid);
                    },
                }
            },
            Err(_) => {
                println!("Failed to execute");
            },
        }
    }
    true
}

fn final_piped_redirect_out(command: Vec<String>, child: Child) -> bool {
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
    if args.len() <= 0 {
        return true
    }
    let mut cmd = Command::new(&args[0]);
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }
    let output = unsafe {
        cmd.args(&args[1..])
            .stdout(Stdio::piped())
            .stdin(Stdio::from_raw_fd(child.stdout
                .expect("No stdout for child process")
                .as_raw_fd()))
            .before_exec(move || {
                let pid = nix::unistd::getpid();
                nix::unistd::setpgid(pid, pid);
                libc::signal(libc::SIGINT, libc::SIG_DFL);
                libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                libc::prctl(1, libc::SIGHUP);
                Result::Ok(())
            })
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

fn final_piped_redirect_out_detached(command: Vec<String>, child: Child) -> bool {
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
    if args.len() <= 0 {
        return true;
    }
    let mut cmd = Command::new(&args[0]);
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }
    let output = unsafe {
        cmd.args(&args[1..])
            .stdout(Stdio::piped())
            .stdin(Stdio::from_raw_fd(child.stdout
                .expect("No stdout for child process")
                .as_raw_fd()))
            .before_exec(move || {
                let pid = nix::unistd::getpid();
                nix::unistd::setpgid(pid, pid);
                libc::signal(libc::SIGINT, libc::SIG_DFL);
                libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                libc::prctl(1, libc::SIGHUP);
                Result::Ok(())
            })
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
    use process::execute::interpret;
}

