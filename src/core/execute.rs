//vec.as_slice() is considered unstable and is subject to change in the future
#![allow(unreachable_code)]
use std::process::*;

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
    unreachable!("I don't know how you did it but dear lord you made it this far".to_string())
}

//Available to interatct with directly for testing purposes
//Highly reccomended the wrapper commands like get_stdout are used
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

#[allow(dead_code)]
fn get_stdout(output: Option<Output>) -> String{
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

#[allow(dead_code)]
fn get_stderr(output: Option<Output>) -> String{
    match output.is_some(){
        true => {
            let temp = output.unwrap();
            return String::from_utf8(temp.stderr).unwrap();
        },
        false => "Please input a valid command".to_string()
    }
}

fn piped(input: Vec<&str>) -> String {
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
    let output = get_stdout(execute(now));
    if later.len() > 0 {
        let output = piped(later);
        return output;
    }
    output
}
//Tests are defunct for now.
#[cfg(test)]
mod tests{
    use std::process::*;
    use super::*;

    #[test]
    fn test_execute(){
        let vec = vec!["echo","hello","there"];
        let test = execute(vec).unwrap();
        let output = Command::new("echo")
                .arg("hello there")
                .output()
                .unwrap_or_else(|e| { 
                    panic!("failed to execute process: {}", e) 
                });
    //Test for Error, Exit Code and Output equality
        assert_eq!(test.status, output.status);
        assert_eq!(String::from_utf8_lossy(&test.stdout),
        String::from_utf8_lossy(&output.stdout));
        assert_eq!(String::from_utf8_lossy(&test.stderr),
        String::from_utf8_lossy(&output.stderr));

    }
    #[test]
    #[should_panic]
    fn test_execute_fail(){
        let vec = vec!["echo","hello","there"];
        let test = execute(vec).unwrap();
        let output = Command::new("/bin/ls")
                .arg("-al")
                .output()
                .unwrap_or_else(|e| { 
                    panic!("failed to execute process: {}", e) 
                });
    //Test for Error, Exit Code and Output equality
        assert_eq!(test.status, output.status);
        assert_eq!(String::from_utf8_lossy(&test.stdout),
        String::from_utf8_lossy(&output.stdout));
        assert_eq!(String::from_utf8_lossy(&test.stderr),
        String::from_utf8_lossy(&output.stderr));

    }

    #[test]
    pub fn test_pipe(){
        //place holder
        assert_eq!(2,2); 
    }
}
