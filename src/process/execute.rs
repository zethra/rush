#![allow(unused_imports)] //Here until interpret is complete
extern crate libc;
extern crate nix;

use std::process::*;
use process::logic::*;
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


#[cfg(test)]
mod tests {
    use super::*;
}

