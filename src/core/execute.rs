//vec.as_slice() is considered unstable and is subject to change in the future
#![allow(unreachable_code)]
use std::process::*;
use std::os::unix::io::{FromRawFd, AsRawFd};

pub fn interpret(command: Vec<&str>) -> String {
//The function that takes a command and interprets what to do with it
//Calls the wrapper functions on execute and pipes commands as needed
    
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
    unreachable!("I don't know how you did it but dear lord you made it this far".to_owned())
}

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

fn piped(input: Vec<&str>) -> String {
    let mut split = split_pipes(input);
    let mut child: Child = first_pipe(split.remove(0));

    while split.len() > 1 {
       child = execute_pipe(split.remove(0), child); 
    }

    final_pipe(split.remove(0), child)
}

fn first_pipe(command: Vec<&str>) -> Child {
    let args = command.as_slice();
    let output = if args.len() > 1 {
            Command::new(&args[0]).args(&args[1.. ])
                .stdout(Stdio::piped()).spawn()
                .ok().expect("Program failed execution 1")
        } else if args.len() == 1{
            Command::new(&args[0])
                .stdout(Stdio::piped()).spawn()
                .ok().expect("Program failed execution 1")
        } else {
            Command::new("")
                .stdout(Stdio::piped()).spawn()
                .ok().expect("Program failed execution 1")
        };
        output
}

fn execute_pipe(command: Vec<&str>, child: Child) -> Child {
    let args = command.as_slice();
    unsafe{
        let output = if args.len() > 1 {
            Command::new(&args[0]).args(&args[1.. ])
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout.unwrap().as_raw_fd()))
                .spawn()
                .ok().expect("Program failed execution 2")
        } else if args.len() == 1{
            Command::new(&args[0])
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout.unwrap().as_raw_fd()))
                .spawn()
                .ok().expect("Program failed execution 2j")
        } else {
            Command::new("")
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout.unwrap().as_raw_fd()))
                .spawn()
                .ok().expect("Program failed execution 2")
        };
        output
    }

}

fn final_pipe(command: Vec<&str>, child: Child) -> String {
    let args = command.as_slice();
    unsafe{
        let output = if args.len() > 1 {
            Command::new(&args[0]).args(&args[1.. ])
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout.unwrap().as_raw_fd()))
                .output().ok()
        } else if args.len() == 1{
            Command::new(&args[0])
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout.unwrap().as_raw_fd()))
                .output().ok()
        } else {
            Command::new("")
                .stdout(Stdio::piped())
                .stdin(Stdio::from_raw_fd(child.stdout.unwrap().as_raw_fd()))
                .output().ok()
        };
        if output.is_some() {
            get_stdout(output)
        } else {
            get_stderr(output)
        }
    }
}


//Tests are defunct for now.
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
