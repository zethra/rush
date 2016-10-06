#![allow(unused_imports)] //Here until interpret is complete
use std::process::*;
use process::logic::*;
use process::stdproc::*;
use process::pipe::*;
use process::ops::*;
use process::pq::*;

///Interpret
///Given an input command, interpret parses and determines what and how
///to execute it and returns output or error output
pub fn interpret(command: String) -> bool {
    let mut op_queues = Opqueue::new();
    let mut proc_queue = Procqueue::new();
    let command: Vec<&str> = command.trim().split(' ').collect();

    //Split order:
    //Split by parallel +=+
    //Split by or ||
    //Split by pipe |
    //Split by and &&
    //Split by (To be expanded)

    let mut pipes = false;
    for i in command.clone() {
        if i.contains('|') && !i.contains("||") {
            pipes = true;
            break;
        }
    }
    if pipes {
        //Pipe or no pipe
        piped(command)
    } else {
        //execute normally
        run(command)
    }
}

///Run
///Runs commands passed to it and returns the output
pub fn run(command: Vec<&str>) -> bool {
    let args = command.as_slice();
    if args.len() > 1 {
        let mut cmd = Command::new(&args[0])
            .args(&args[1..])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Command failed to start");
        let status = cmd.wait().expect("failed to wait for child");;
        status.success()
    } else if args.len() == 1 {
        let mut cmd = Command::new(&args[0])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Command failed to start");
        let status = cmd.wait().expect("failed to wait for child");
        status.success()
    } else {
        let mut cmd = Command::new("")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Command failed to start");
        let status = cmd.wait().expect("failed to wait for child");
        status.success()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_test() {
        let vec = "ls -al".to_owned();
        let result = interpret(vec);
        assert!(!result.is_empty());
    }

    #[test]
    fn execute_fail() {
        let vec = "blah".to_owned();
        let result = interpret(vec);
        assert_eq!("Please input a valid command",result);
    }
}

