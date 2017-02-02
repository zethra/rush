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

///First Pipe
///Always executed if piping and returns the child process to be used
///for the next pipe.
pub fn first_pipe(command: &String, args: &Vec<String>) -> io::Result<Child> {
    let args = args.as_slice();
    // TODO Handle args having length 0
    let mut cmd = Command::new(command);
    if args.len() > 0 {
        cmd.args(&args);
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
pub fn execute_pipe(command: &String, args: &Vec<String>, child: Child) -> io::Result<Child> {
    let args = args.as_slice();
    // TODO Handle args having length 0
    let mut cmd = Command::new(command);
    if args.len() > 0 {
        cmd.args(&args);
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
pub fn final_pipe(command: &String, args: &Vec<String>, child: Child) -> bool {
    let args = args.as_slice();
    let mut cmd = Command::new(command);
    if args.len() > 0 {
        cmd.args(&args);
    }
    unsafe {
        match cmd.stdout(Stdio::inherit())
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

