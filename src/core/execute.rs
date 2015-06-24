//vec.as_slice() is considered unstable and is subject to change in the future
#![allow(unreachable_code)]
#![allow(unused_variables)] //temporary until piping is completed
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
    unreachable!("I don't know how you did it but dear lord you made it this far".to_owned())
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
        false => "Please input a valid command".to_owned()
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
        false => "Please input a valid command".to_owned()
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
fn split_pipes(input: Vec<&str>) -> Vec<Vec<&str>> {
    let input_slice = input.as_slice();
    let mut thing: Vec<Vec<&str>> = Vec::new();
    let mut temp: Vec<&str> = Vec::new();
    for i in input_slice {
        if i.contains('|') {
            let mut splits = i.split('|');
            &mut temp.push(splits.next().unwrap());
            if temp.last().unwrap() == &""{
                temp.pop();
            }
            &mut thing.push(temp.clone());
            &mut temp.clear();
            &mut temp.push(splits.next().unwrap());
            if temp.last().unwrap() == &""{
                temp.pop();
            }
        } else {
            &mut temp.push(i);
            if temp.last().unwrap() == &""{
                temp.pop();
            }
        }
    }
    &mut thing.push(temp);
    thing
}

fn piped(input: Vec<&str>) -> String {
    let split = split_pipes(input); 
    "Blah".to_owned()
}

#[allow(dead_code)]
fn execute_pipe(command: Vec<&str>) -> Option<Output>{
   Command::new("").output().ok()
}

//Tests are defunct for now.
#[cfg(test)]
mod tests{
    use std::process::*;
    use super::*;
    use std::os::unix::io::AsRawFd;
    use std::os::unix::io::FromRawFd;
    #[test]
    fn pipes(){
        let cmd = Command::new("ls").arg("/")
                    .stdout(Stdio::piped())
                    .spawn();
    unsafe{ 
        let cmd2 = Command::new("grep").arg("etc")
                    .stdin(Stdio::from_raw_fd(cmd.ok().unwrap().stdout.unwrap().as_raw_fd()))
                    .output().ok();
        assert_eq!("etc", get_stdout(cmd2).trim());   
     }
    }
}
