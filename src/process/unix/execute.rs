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
use self::nix::unistd::Pid;
use self::nix::sys::signal;
use self::nix::sys::signal::{SigAction, SigHandler, SaFlags, SigSet, sigaction};

/// Run
/// Runs commands passed to it and returns whether the command exited successfully.
pub fn run(command: &String, args: &Vec<String>, vars: &Vec<(String, Option<String>)>) -> bool {
    let mut cmd = Command::new(command);
    let args = args.as_slice();
    if args.len() > 0 {
        cmd.args(args.iter());
    }
    for var in vars {
        match &var.1 {
            &Some(ref v) => cmd.env(&var.0, &v),
            &None => cmd.env(&var.0, ""),
        };
    }
    match cmd.stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .before_exec(move || {
            let pid = nix::unistd::getpid();
            nix::unistd::setpgid(pid, pid);
            let hdl = SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty());
            unsafe {
                sigaction(signal::SIGINT, &hdl).unwrap();
                sigaction(signal::SIGQUIT, &hdl).unwrap();
                sigaction(signal::SIGTSTP, &hdl).unwrap();
                sigaction(signal::SIGTTIN, &hdl).unwrap();
                sigaction(signal::SIGTTOU, &hdl).unwrap();
                sigaction(signal::SIGTSTP, &hdl).unwrap();
                libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGHUP);
            }
            Result::Ok(())
        })
        .spawn() {
        Ok(mut child) => {
            let child_pgid = child.id() as i32;
            if nix::unistd::tcsetpgrp(0, Pid::from_raw(child_pgid)).is_err() {
                return false;
            }
            match child.wait() {
                Ok(status) => {
                    if nix::unistd::tcsetpgrp(0, nix::unistd::getpid()).is_err() {
                        return false;
                    }
                    status.success()
                }
                Err(e) => {
                    if nix::unistd::tcsetpgrp(0, nix::unistd::getpid()).is_err() {
                        return false;
                    }
                    println!("{}", e);
                    false
                }
            }
        }
        Err(e) => {
            println!("{}", e);
            false
        }
    }
}

// Run Detached
// Like Run but runs the command deteched from the console.
pub fn run_detached(command: &String,
                    args: &Vec<String>,
                    vars: &Vec<(String, Option<String>)>)
                    -> bool {
    let mut cmd = Command::new(command);
    let args = args.as_slice();
    if args.len() > 0 {
        cmd.args(args.iter());
    }
    for var in vars {
        match &var.1 {
            &Some(ref v) => cmd.env(&var.0, &v),
            &None => cmd.env(&var.0, ""),
        };
    }
    match cmd.stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .before_exec(move || {
            let pid = nix::unistd::getpid();
            nix::unistd::setpgid(pid, pid);
            let hdl = SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty());
            unsafe {
                sigaction(signal::SIGINT, &hdl).unwrap();
                sigaction(signal::SIGQUIT, &hdl).unwrap();
                sigaction(signal::SIGTSTP, &hdl).unwrap();
                sigaction(signal::SIGTTIN, &hdl).unwrap();
                sigaction(signal::SIGTTOU, &hdl).unwrap();
                sigaction(signal::SIGTSTP, &hdl).unwrap();
                libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGHUP);
            }
            Result::Ok(())
        })
        .spawn() {
        Ok(mut child) => {
            let child_pgid = child.id() as i32;
            println!("{}", child_pgid);
            thread::spawn(move || match child.wait() {
                Ok(status) => {
                    if status.success() {
                        println!("+ {} done", child_pgid);
                    } else {
                        match status.code() {
                            Some(c) => println!("+ {} exit {}", child_pgid, c),
                            None => println!("+ {} error", child_pgid),
                        }
                    }
                    status.success()
                }
                Err(e) => {
                    println!("+ {} {}", child_pgid, e);
                    false
                }
            });
            true
        }
        Err(e) => {
            println!("{}", e);
            false
        }
    }
}

// Redirect Out
// Like Run but redirects commands stdout to a file.
pub fn redirect_out(command: &String,
                    args: &Vec<String>,
                    vars: &Vec<(String, Option<String>)>,
                    file_path: &String,
                    fd: &i32)
                    -> bool {
    let path = Path::new(&file_path);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut cmd = Command::new(command);
    if args.len() > 0 {
        cmd.args(args.iter());
    }
    for var in vars {
        match &var.1 {
            &Some(ref v) => cmd.env(&var.0, &v),
            &None => cmd.env(&var.0, ""),
        };
    }
    match *fd {
        1 => {
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::inherit());
        }
        2 => {
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::piped());
        }
        _ => {
            println!("Unknown file desctiptor for redirect");
            return false;
        }
    }
    match cmd.before_exec(move || {
            let pid = nix::unistd::getpid();
            nix::unistd::setpgid(pid, pid);
            let hdl = SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty());
            unsafe {
                sigaction(signal::SIGINT, &hdl).unwrap();
                sigaction(signal::SIGQUIT, &hdl).unwrap();
                sigaction(signal::SIGTSTP, &hdl).unwrap();
                sigaction(signal::SIGTTIN, &hdl).unwrap();
                sigaction(signal::SIGTTOU, &hdl).unwrap();
                sigaction(signal::SIGTSTP, &hdl).unwrap();
                libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGHUP);
            }
            Result::Ok(())
        })
        .spawn() {
        Ok(child) => {
            let child_pgid = child.id() as i32;
            if nix::unistd::tcsetpgrp(0, Pid::from_raw(child_pgid)).is_err() {
                return false;
            }
            match child.wait_with_output() {
                Ok(output) => {
                    if nix::unistd::tcsetpgrp(0, nix::unistd::getpid()).is_err() {
                        return false;
                    }
                    match *fd {
                        1 => {
                            if let Err(e) = file.write_all(output.stdout.as_slice()) {
                                println!("Couldn't write to {}: {}", display, e.description());
                                return false;
                            }
                        }
                        2 => {
                            if let Err(e) = file.write_all(output.stderr.as_slice()) {
                                println!("Couldn't write to {}: {}", display, e.description());
                                return false;
                            }
                        }
                        _ => {
                            println!("Unknown file desctiptor for redirect");
                            return false;
                        }
                    }
                    return output.status.success();
                }
                Err(e) => {
                    if nix::unistd::tcsetpgrp(0, nix::unistd::getpid()).is_err() {
                        return false;
                    }
                    println!("{}", e);
                    false
                }
            }
        }
        Err(e) => {
            println!("{}", e);
            false
        }
    }
}

// Redirect Out Detached
// Like Run but runs the command deteched from the console
// and redirects commands stdout to a file.
pub fn redirect_out_detached(command: &String,
                             args: &Vec<String>,
                             vars: &Vec<(String, Option<String>)>,
                             file_path: &String,
                             fd: &i32)
                             -> bool {
    let fd = fd.clone();
    let path = Path::new(&file_path);
    let display = path.display();
    match File::create(&path) {
        Ok(_) => {}
        Err(e) => {
            println!("Couldn't open {}: {}", display, e.description());
            return false;
        }
    }
    let file_path = file_path.clone();
    let mut cmd = Command::new(command);
    if args.len() > 0 {
        cmd.args(args.iter());
    }
    for var in vars {
        match &var.1 {
            &Some(ref v) => cmd.env(&var.0, &v),
            &None => cmd.env(&var.0, ""),
        };
    }
    match fd {
        1 => {
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::inherit());
        }
        2 => {
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::piped());
        }
        _ => {
            println!("Unknown file desctiptor for redirect");
            return false;
        }
    }
    match cmd.before_exec(move || {
            let pid = nix::unistd::getpid();
            nix::unistd::setpgid(pid, pid);
            let hdl = SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty());
            unsafe {
                sigaction(signal::SIGINT, &hdl).unwrap();
                sigaction(signal::SIGQUIT, &hdl).unwrap();
                sigaction(signal::SIGTSTP, &hdl).unwrap();
                sigaction(signal::SIGTTIN, &hdl).unwrap();
                sigaction(signal::SIGTTOU, &hdl).unwrap();
                sigaction(signal::SIGTSTP, &hdl).unwrap();
                libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGHUP);
            }
            Result::Ok(())
        })
        .spawn() {
        Ok(child) => {
            let child_pgid = child.id() as i32;
            println!("{}", child_pgid);
            thread::spawn(move || match child.wait_with_output() {
                Ok(output) => {
                    let path = Path::new(&file_path);
                    let display = path.display();
                    let mut file = match File::create(&path) {
                        Ok(file) => file,
                        Err(e) => {
                            println!("Couldn't open {}: {}", display, e.description());
                            return false;
                        }
                    };
                    match fd {
                        1 => {
                            if let Err(e) = file.write_all(output.stdout.as_slice()) {
                                println!("+ {} error", child_pgid);
                                println!("Couldn't write to {}: {}", display, e.description());
                                return false;
                            }
                        }
                        2 => {
                            if let Err(e) = file.write_all(output.stderr.as_slice()) {
                                println!("+ {} error", child_pgid);
                                println!("Couldn't write to {}: {}", display, e.description());
                                return false;
                            }
                        }
                        _ => {
                            println!("+ {} error", child_pgid);
                            println!("Unknown file desctiptor for redirect");
                            return false;
                        }
                    }
                    if output.status.success() {
                        println!("+ {} done", child_pgid);
                    } else {
                        match output.status.code() {
                            Some(c) => println!("+ {} exit {}", child_pgid, c),
                            None => println!("+ {} error", child_pgid),
                        }
                    }
                    output.status.success()
                }
                Err(e) => {
                    println!("{}", e);
                    false
                }
            });
            true
        }
        Err(e) => {
            println!("{}", e);
            false
        }
    }
}
