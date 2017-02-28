extern crate libc;
extern crate nix;

use std::process::*;
use std::io;
#[cfg(unix)]
use process::unix::execute;
#[cfg(unix)]
use process::unix::pipe;
#[cfg(windows)]
use process::windows::execute;
#[cfg(windows)]
use process::windows::pipe;

pub fn run(command: &String, args: &Vec<String>, vars: &Vec<(String, Option<String>)>) -> bool {
    execute::run(command, args, vars)
}

pub fn redirect_out(command: &String, args: &Vec<String>, vars: &Vec<(String, Option<String>)>, file_path: &String) -> bool {
    execute::redirect_out(command, args, vars, file_path)
}

pub fn first_pipe(command: &String, args: &Vec<String>, vars: &Vec<(String, Option<String>)>) -> io::Result<Child> {
    pipe::first_pipe(command, args, vars)
}

pub fn execute_pipe(command: &String, args: &Vec<String>, vars: &Vec<(String, Option<String>)>, child: Child) -> io::Result<Child> {
    pipe::execute_pipe(command, args, vars, child)
}

pub fn final_pipe(command: &String, args: &Vec<String>, vars: &Vec<(String, Option<String>)>, child: Child) -> bool {
    pipe::final_pipe(command, args, vars, child)
}

pub fn final_piped_redirect_out(command: &String, args: &Vec<String>, vars: &Vec<(String, Option<String>)>, child: Child, file_path: &String) -> bool {
    pipe::final_piped_redirect_out(command, args, vars, child, file_path)
}