#![allow(unreachable_code)]

use std::process::*;
use std::os::unix::io::{FromRawFd, AsRawFd};
use std::io::Result;

///Interpret
///Given an input command, interpret parses and determines what and how
///to execute it and returns output or error output
pub fn interpret(command: Vec<&str>) -> String {

    let mut pipes = false;
    for i in command.clone() {
       if i.contains('|') {
           pipes = true;
           break;
        }
    }

    if pipes { //Pipe or no pipe
        let output = piped(command);
        return output;
    } else { //execute normally
        let output = execute(command);
        let out = get_stdout(output.clone());
        if out.is_empty(){
            return get_stderr(output.clone());
        }
        return out;
    }
    unreachable!("I don't know how you did it but dear
                  lord you made it this far".to_owned())
}

///Execute
///Runs commands passed to it and returns the output
fn execute(command: Vec<&str>) -> Option<Output>{
    let args = command.as_slice();
    let output = if args.len() > 1 {
            Command::new(&args[0]).args(&args[1.. ]).output().ok()
        } else if args.len() == 1{
            Command::new(&args[0]).output().ok()
        } else {
            Command::new("").output().ok()
        };
        output
 }

///Get Stdout
///Returns the standard output of an executed command or returns that the
///command was invalid
fn get_stdout(output: Option<Output>) -> String{
    match output.is_some(){
        true => {
            let temp = output.unwrap();
            return String::from_utf8(temp.stdout).unwrap();
        },
        false => "Please input a valid command".to_owned()
            //Used in order to work out for the Option input
            //However with process stderr is used for better
            //outputs of messages
    }
}

///Get Stderr
///Returns standard error output of an executed command or returns that
///command was invalid
fn get_stderr(output: Option<Output>) -> String{
    match output.is_some(){
        true => {
            let temp = output.unwrap();
            return String::from_utf8(temp.stderr).unwrap();
        },
        false => "Please input a valid command".to_owned()
    }
}

#[allow(dead_code)] //At least until I find a use for it
fn get_status(output: Option<Output>) -> bool{
    match output.is_some(){
        true => {
            let temp = output.unwrap();
            return temp.status.success();
        },
        false => false,
    }
}

///Split Pipes
///Given a command with pipes in it, it will split them into separate
///commands to be executed
fn split_pipes(input: Vec<&str>) -> Vec<Vec<&str>> {
    let input_slice = input.as_slice();
    let mut thing: Vec<Vec<&str>> = Vec::new();
    let mut temp: Vec<&str> = Vec::new();
    for i in input_slice {
        if i.contains('|') {
            let mut splits = i.split('|');
            temp.push(splits.next().unwrap());
            if temp.last().unwrap() == &""{
                temp.pop();
            }
            thing.push(temp.clone());
            temp.clear();
            temp.push(splits.next().unwrap());
            if temp.last().unwrap() == &""{
                temp.pop();
            }
        } else {
            temp.push(i);
        }
    }
    thing.push(temp);
    thing
}

///Piped
///The logic of piping is done here and calls the functions that execute
///the pipes and returns the result
fn piped(input: Vec<&str>) -> String {
    let mut split = split_pipes(input);
    let mut child_result = first_pipe(split.remove(0));
    let mut child: Child;

    //Error handling done in here rather than the functions they call
    //Code is unwrapped seeing that if there is no error then it must
    //be safe function wise

    if child_result.is_ok() {
       child = child_result.ok().unwrap();
    } else {
        return child_result.err().unwrap().to_string();
    }
    while split.len() > 1 {
        child_result = execute_pipe(split.remove(0), child);
        if child_result.is_ok() {
            child = child_result.ok().unwrap();
        } else {
            return child_result.err().unwrap().to_string();
        }
    }

    final_pipe(split.remove(0), child)
}

///First Pipe
///Always executed if piping and returns the child process to be used
///for the next pipe.
fn first_pipe(command: Vec<&str>) -> Result<Child> {
    let args = command.as_slice();

    let output = if args.len() > 1 {
            Command::new(&args[0]).args(&args[1.. ])
                .stdout(Stdio::piped()).spawn()
        } else if args.len() == 1{
            Command::new(&args[0])
                .stdout(Stdio::piped()).spawn()
        } else {
            Command::new("")
                .stdout(Stdio::piped()).spawn()
        };

    output
}

///Execute Pipe
///Used if there are more than two commands with piping. Takes a Child process
///as input for the next pipe and returns a Child process.
fn execute_pipe(command: Vec<&str>, child: Child) -> Result<Child> {
    let args = command.as_slice();
    unsafe{
        let output = if args.len() > 1 {
            Command::new(&args[0]).args(&args[1.. ])
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout.unwrap().as_raw_fd()))
                .spawn()
        } else if args.len() == 1{
            Command::new(&args[0])
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout.unwrap().as_raw_fd()))
                .spawn()
        } else {
            Command::new("")
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout.unwrap().as_raw_fd()))
                .spawn()
        };
        output
    }

}

///Final Pipe
///Always executed when piping processes. Takes a child process as input
///and returns the output of piping the commands.
fn final_pipe(command: Vec<&str>, child: Child) -> String {
    let args = command.as_slice();
    unsafe{
        let output = if args.len() > 1 {
            Command::new(&args[0]).args(&args[1.. ])
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout.unwrap().as_raw_fd()))
                .output()
        } else if args.len() == 1{
            Command::new(&args[0])
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout.unwrap().as_raw_fd()))
                .output()
        } else {
            Command::new("")
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout.unwrap().as_raw_fd()))
                .output()
        };
    //Get rid of ok() and do error handling here
        if output.is_ok() {
            get_stdout(output.ok())
        } else {
            output.err().unwrap().to_string()
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn pipes() {
        let vec: Vec<&str> = "ls /|grep bin| sort -r"
            .trim().split(' ').collect();
        let result = interpret(vec);
        assert_eq!("sbin\nbin\n",result);
     }

    #[test]
    #[should_panic]
    fn pipes_fail() {
        let vec: Vec<&str> = "ls |grep bin| sort -r"
            .trim().split(' ').collect();
        let result = interpret(vec);
        assert_eq!("Please input a valid command",result);
    }

    #[test]
    fn execute(){
        let vec: Vec<&str> = "ls -al"
            .trim().split(' ').collect();
        let result = interpret(vec);
        assert!(!result.is_empty());

    }

    #[test]
    fn execute_fail(){
        let vec: Vec<&str> = "blah"
            .trim().split(' ').collect();
        let result = interpret(vec);
        assert_eq!("Please input a valid command",result);
    }
}
