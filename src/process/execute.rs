#![allow(unused_imports)] //Here until interpret is complete
use std::process::*;
use process::logic::*;
use process::stdproc::*;
use process::pipe::*;
use process::ops::*;

///Interpret
///Given an input command, interpret parses and determines what and how
///to execute it and returns output or error output
pub fn interpret(command: String) -> String {
    let mut queue = Opqueue::new();
    let command: Vec<&str> = command.trim().split(' ').collect();
    //Refactoring
    //Break commands by logic
    //Run commands by logic precedence by looping through all of them here
    //output results
    //Create more functions for dealing with ands etc.
    let mut pipes = false;
    for i in command.clone() {
       if i.contains('|') {
           pipes = true;
           break;
        }
    }
    let output: Option<Output>;
    if pipes { //Pipe or no pipe
        output = piped(command);
    } else { //execute normally
        output = run(command);
    }

    get_stdout_or_stderr(output)
}

///Run
///Runs commands passed to it and returns the output
pub fn run(command: Vec<&str>) -> Option<Output>{
    let args = command.as_slice();
    if args.len() > 1 {
        Command::new(&args[0]).args(&args[1.. ]).output().ok()
    } else if args.len() == 1{
        Command::new(&args[0]).output().ok()
    } else {
        Command::new("").output().ok()
    }
 }

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn execute_test(){
        let vec = "ls -al".to_owned();
        let result = interpret(vec);
        assert!(!result.is_empty());

    }

    #[test]
    fn execute_fail(){
        let vec = "blah".to_owned();
        let result = interpret(vec);
        assert_eq!("Please input a valid command",result);
    }

}

