//vec.as_slice() is considered unstable and is subject to change in the future

use std::process::*;

//To put stdout or stderr into a string use String::from_utf8_lossy(&variable.stdout or stderr)
//For status code just use variable.status

//Available to interatct with directly for testing purposes
//Highly reccomended the wrapper commands like get_stdout are used

pub fn execute(command: Vec<&str>) -> Option<Output>{
    //Check for input before using execute as nothing but spaces crashes
    //trim() before input
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

pub fn get_stdout(command: Vec<&str>) -> String{
    let output = execute(command);
    match output.is_some(){
        true => {
            let temp = output.unwrap();
            return String::from_utf8(temp.stdout).unwrap();
        },
        false => "Please input a valid command".to_string()
    }
}
//pub fn pipe(){};

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
}
