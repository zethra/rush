#![allow(unused_imports)] //Here until interpret is complete

use std::process::*;
use process::stdproc::*;
use process::windows::pipe::*;
use process::ops::*;
use process::pq::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

///Run
///Runs commands passed to it and returns the output
pub fn run(command: Vec<&str>) -> bool {
    let args = command.as_slice();
    if args.len() > 1 {
        match Command::new(&args[0])
            .args(&args[1..])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
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
    } else if args.len() == 1 {
        match Command::new(&args[0])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
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

