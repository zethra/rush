//vec.as_slice() is considered unstable and is subject to change in the future
#![allow(unreachable_code)]
use std::process::*;
use std::os::unix::io::FromRawFd;//unsafe
use std::os::unix::io::AsRawFd;//Unsafe

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
        let output = piped(command, true);
        return output;
    } else { //execute normally
        let output = execute(command);
        let out = get_stdout(output.clone());
        if out.is_empty(){
            return get_stderr(output.clone());
        }
        return out;
    }
    unreachable!("I don't know how you did it but dear lord you made it this far".to_string())
}

pub fn execute(command: Vec<&str>) -> Option<Output>{
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

pub fn get_stdout(output: Option<Output>) -> String{
    match output.is_some(){
        true => {
            let temp = output.unwrap();
            return String::from_utf8(temp.stdout).unwrap();
        },
        false => "Please input a valid command".to_string()
            //Used in order to work out for the Option input
            //However with process stderr is used for better
            //outputs of messages
    }
}

pub fn get_stderr(output: Option<Output>) -> String{
    match output.is_some(){
        true => {
            let temp = output.unwrap();
            return String::from_utf8(temp.stderr).unwrap();
        },
        false => "Please input a valid command".to_string()
    }
}

pub fn get_status(output: Option<Output>) -> bool{
    match output.is_some(){
        true => {
            let temp = output.unwrap();
            return temp.status.success();
        },
        false => false,
    }
}

fn piped(input: Vec<&str>, first_pass: bool) -> String {
    
    let input_slice = input.as_slice();
    let mut now: Vec<&str> = Vec::new();
    let mut later: Vec<&str> = Vec::new();
    let mut piping = false;
    
    for i in input_slice {
        if i.contains('|') || piping == true{
            piping = true;
            let mut split:Vec<&str> = Vec::new();
            let split_inputs = i.split("|");
            for j in split_inputs {
                split.push(j);
            }
            match split.len() {
                0 => continue,
                1 => {
                    let temp = split.pop().unwrap();
                    if temp != ""{ 
                        later.push(temp);
                    }
                }
                2 => {
                    let later_push = split.pop().unwrap();
                    let now_push = split.pop().unwrap();
                    if later_push != ""{
                        later.push(later_push);
                    }
                    if now_push != ""{
                        now.push(now_push);
                    }
                },
                _ => unreachable!("Splitting one command should not be more than 2 in it's length")
            }
        } else {
           now.push(i);
        }

    }

    //gets output for next pprogram until there are no more programs to execute
    //outputs final text
    //How do I get stdin for execute_pipe in the following block of code?
    let executed = execute_pipe(now);

    if get_stdout(executed.clone()).len() == 0 {
        return get_stderr(executed.clone())
    }

    if later.len() > 0 {
        return piped(later, false)
    }

    get_stdout(executed.clone())
}

//Given an input and stdin create the output for next pipe
//INCOMPLETED
//I need to figure out how the Stdio module works in relation to the command module
//Maybe I need to return a tuple the Option<Output> and Stdio
//need to use spawn command not output command!
fn execute_pipe(command: Vec<&str>) -> Option<Output>{
   Command::new("").output().ok()
}


//Tests are defunct for now.
#[cfg(test)]
mod tests{
    use std::os::unix::io::FromRawFd;//unsafe
    use std::os::unix::io::AsRawFd;//Unsafe
    use std::process::*;
    use super::*;
    #[test]
    fn pipes(){
        let first = Command::new("ls").arg("/").stdout(Stdio::piped()).spawn().ok();
        unsafe {
            let second = Command::new("grep").arg("etc").stdin(Stdio::inherit(first)).output().ok();
            assert_eq!(get_stdout(second),"etc");
        }
    }
}
