extern crate libc;
extern crate nix;

use std::process::*;
use process::ops::*;
use std::io;
#[cfg(unix)]
use process::unix::execute;
#[cfg(unix)]
use process::unix::pipe;
#[cfg(windows)]
use process::windows::execute;
#[cfg(windows)]
use process::windows::pipe;

pub fn run(command: &String, args: &Vec<String>) -> bool {
    execute::run(command, args)
}

pub fn first_pipe(command: &String, args: &Vec<String>) -> io::Result<Child> {
    pipe::first_pipe(command, args)
}

pub fn execute_pipe(command: &String, args: &Vec<String>, child: Child) -> io::Result<Child> {
    pipe::execute_pipe(command, args, child)
}

pub fn final_pipe(command: &String, args: &Vec<String>, child: Child) -> bool {
    pipe::final_pipe(command, args, child)
}