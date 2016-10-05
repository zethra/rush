use std::process::{Stdio, Command, Child, Output};
use std::os::unix::io::{FromRawFd, AsRawFd};
use std::io::Result;

///Split Pipes
///Given a command with pipes in it, it will split them into separate
///commands to be executed
fn split_pipes(input: Vec<&str>) -> Vec<Vec<&str>> {
    let input_slice = input.as_slice();
    let mut pipe_commands: Vec<Vec<&str>> = Vec::new();
    let mut temp: Vec<&str> = Vec::new();
    for i in input_slice {
        if i.contains('|') {
            let mut splits = i.split('|');
            temp.push(splits.next()
                .expect("Failed to split pipes"));
            if temp.last()
                .expect("Called last on an empty vec") == &"" {
                temp.pop();
            }
            pipe_commands.push(temp.clone());
            temp.clear();
            temp.push(splits.next()
                .expect("Unwrapped a non existent value"));
            if temp.last()
                .expect("Unwrapped an empty list") == &"" {
                temp.pop();
            }
        } else {
            temp.push(i);
        }
    }
    pipe_commands.push(temp);
    pipe_commands
}

///Piped
///The logic of piping is done here and calls the functions that execute
///the pipes and returns the result
pub fn piped(input: Vec<&str>) -> Option<Output> {
    let mut split = split_pipes(input);
    let mut child_result = first_pipe(split.remove(0));
    let mut child: Child;

    child = child_result.ok().expect("Failed to unwrap an Result");

    while split.len() > 1 {
        child_result = execute_pipe(split.remove(0), child);
        child = child_result.ok().expect("Failed to unwrap an Result");
    }

    final_pipe(split.remove(0), child)
}

///First Pipe
///Always executed if piping and returns the child process to be used
///for the next pipe.
fn first_pipe(command: Vec<&str>) -> Result<Child> {
    let args = command.as_slice();
    if args.len() > 1 {
        Command::new(&args[0]).args(&args[1..])
            .stdout(Stdio::piped()).spawn()
    } else if args.len() == 1 {
        Command::new(&args[0])
            .stdout(Stdio::piped()).spawn()
    } else {
        Command::new("")
            .stdout(Stdio::piped()).spawn()
    }
}

///Execute Pipe
///Used if there are more than two commands with piping. Takes a Child process
///as input for the next pipe and returns a Child process.
fn execute_pipe(command: Vec<&str>, child: Child) -> Result<Child> {
    let args = command.as_slice();
    unsafe {
        let output = if args.len() > 1 {
            Command::new(&args[0]).args(&args[1..])
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout
                    .expect("No stdout").as_raw_fd()))
                .spawn()
        } else if args.len() == 1 {
            Command::new(&args[0])
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout
                    .expect("No stdout").as_raw_fd()))
                .spawn()
        } else {
            Command::new("")
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout
                    .expect("No stdout").as_raw_fd()))
                .spawn()
        };
        output
    }
}

///Final Pipe
///Always executed when piping processes. Takes a child process as input
///and returns the output of piping the commands.
fn final_pipe(command: Vec<&str>, child: Child) -> Option<Output> {
    let args = command.as_slice();
    unsafe {
        let output = if args.len() > 1 {
            Command::new(&args[0]).args(&args[1..])
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout
                    .expect("No stdout for child process")
                    .as_raw_fd()))
                .output()
        } else if args.len() == 1 {
            Command::new(&args[0])
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout
                    .expect("No stdout for child process")
                    .as_raw_fd()))
                .output()
        } else {
            Command::new("")
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout
                    .expect("No stdout for child process")
                    .as_raw_fd()))
                .output()
        };
        output.ok()
    }
}

#[cfg(test)]
mod tests {
    use process::execute::interpret;

    #[test]
    fn pipes() {
        let vec = "ls /|grep bin| sort -r".to_owned();
        let result = interpret(vec);
        assert_eq!("sbin\nbin\n",result);
    }

    #[test]
    #[should_panic]
    fn pipes_fail() {
        let vec = "ls |grep bin| sort -r".to_owned();
        let result = interpret(vec);
        assert_eq!("Please input a valid command",result);
    }
}

