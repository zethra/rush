#![allow(unused_must_use)]
#![allow(dead_code)]
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

/// First Pipe
/// Always executed if piping and returns the child process to be used
/// for the next pipe.
pub fn first_pipe(command: &String,
                  args: &Vec<String>,
                  vars: &Vec<(String, Option<String>)>)
                  -> io::Result<Child> {
    let args = args.as_slice();
    // TODO Handle args having length 0
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

/// Execute Pipe
/// Used if there are more than two commands with piping. Takes a Child process
/// as input for the next pipe and returns a Child process.
pub fn execute_pipe(command: &String,
                    args: &Vec<String>,
                    vars: &Vec<(String, Option<String>)>,
                    child: Child)
                    -> io::Result<Child> {
    let args = args.as_slice();
    // TODO Handle args having length 0
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
    unsafe {
        cmd.stdout(Stdio::piped())
            .stdin(Stdio::from_raw_fd(child.stdout
                .expect("No stdout")
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
            .spawn()
    }
}

/// Final Pipe
/// Always executed when piping processes. Takes a child process as input
/// and returns the whether the command exited successfully.
pub fn final_pipe(command: &String,
                  args: &Vec<String>,
                  vars: &Vec<(String, Option<String>)>,
                  child: Child)
                  -> bool {
    let args = args.as_slice();
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
                    }
                    Err(e) => {
                        nix::unistd::tcsetpgrp(0, nix::unistd::getpid());
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
}

// Final Pipe Deteched
// Like Final Pipe but runs the command deteched from the console.
pub fn final_pipe_detached(command: &String,
                           args: &Vec<String>,
                           vars: &Vec<(String, Option<String>)>,
                           child: Child)
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
                println!("{}", child_pgid);
                thread::spawn(move || {
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
                            status.success()
                        }
                        Err(e) => {
                            println!("+ {} {}", child_pgid, e);
                            false
                        }
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
}

// Final Pipe Redirect Out
// Like Final Pipe but redirects commands stdout to a file.
pub fn final_piped_redirect_out(command: &String,
                                args: &Vec<String>,
                                vars: &Vec<(String, Option<String>)>,
                                file_path: &String,
                                child: Child)
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
    unsafe {
        match cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped())
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
            Ok(child) => {
                let child_pgid = child.id() as i32;
                if nix::unistd::tcsetpgrp(0, child_pgid).is_err() {
                    return false;
                }
                match child.wait_with_output() {
                    Ok(output) => {
                        if nix::unistd::tcsetpgrp(0, nix::unistd::getpid()).is_err() {
                            return false;
                        }
                        if let Err(e) = file.write_all(output.stdout.as_slice()) {
                            println!("Couldn't write to {}: {}", display, e.description());
                            return false;
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
}

// Final Pipe Redirect Out Deteched
// Like Final Pipe but runs the command deteched from the console
// and redirects commands stdout to a file.
pub fn final_piped_redirect_out_detached(command: &String,
                                         args: &Vec<String>,
                                         vars: &Vec<(String, Option<String>)>,
                                         file_path: &String,
                                         child: Child)
                                         -> bool {
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
    unsafe {
        match cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped())
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
            Ok(child) => {
                let child_pgid = child.id() as i32;
                println!("{}", child_pgid);
                thread::spawn(move || {
                    match child.wait_with_output() {
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
                            if let Err(e) = file.write_all(output.stdout.as_slice()) {
                                println!("+ {} Couldn't write to {}: {}",
                                         child_pgid,
                                         display,
                                         e.description());
                                return false;
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
}
