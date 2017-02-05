#![allow(unused_must_use)]
extern crate libc;
extern crate nix;

use std::process::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::os::unix::process::CommandExt;
use std::thread;

///Interpret
///Given an input command, interpret parses and determines what and how
///to execute it and returns output or error output

///Run
///Runs commands passed to it and returns the output
pub fn run(command: &String, args: &Vec<String>, vars: &Vec<(String, Option<String>)>) -> bool {
    let mut cmd = Command::new(command);
    let args = args.as_slice();
    if args.len() > 0 {
        cmd.args(&args);
    }
    for var in vars {
        match &var.1 {
            &Some(ref v) => cmd.env(&var.0, &v),
            &None => cmd.env(&var.0, ""),
        };
    }
    match cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
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
        .spawn() {
        Ok(mut child) => {
            let child_pgid = child.id() as i32;
            if nix::unistd::tcsetpgrp(0, child_pgid).is_err() { return false; }
            match child.wait() {
                Ok(status) => {
                    if nix::unistd::tcsetpgrp(0, nix::unistd::getpid()).is_err() { return false; }
                    status.success()
                },
                Err(_) => {
                    if nix::unistd::tcsetpgrp(0, nix::unistd::getpid()).is_err() { return false; }
                    println!("Failed to wait for child");
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


pub fn run_detached(command: Vec<String>) -> bool {
    thread::spawn(move || {
        let args = command.as_slice();
        if args.len() <= 0 {
            return;
        }
        let mut cmd = Command::new(&args[0]);
        if args.len() > 1 {
            cmd.args(&args[1..]);
        }
        match cmd
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
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
    });
    true
}

pub fn redirect_out(command: &String, args: &Vec<String>, vars: &Vec<(String, Option<String>)>, file_path: &String) -> bool {
    let mut cmd = Command::new(command);
    if args.len() > 0 {
        cmd.args(&args);
    }
    let output = cmd.before_exec(move || {
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
        .output()
        .ok();
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
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(str_out.as_bytes()) {
        panic!("Couldn't write to {}: {}", display, why.description());
    }
    true
}

pub fn redirect_out_detached(command: Vec<String>) -> bool {
    thread::spawn(move || {
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
            return;
        }
        let mut cmd = Command::new(&args[0]);
        if args.len() > 1 {
            cmd.args(&args[1..]);
        }
        let output = cmd.before_exec(move || {
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
            .output()
            .ok();
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
            Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };
        if let Err(why) = file.write_all(str_out.as_bytes()) {
            panic!("Couldn't write to {}: {}", display, why.description());
        }
    });
    true
}