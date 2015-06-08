//vec.as_slice() is considered unstable and is subject to change in the future

use std::process::*;

//To put stdout or stderr into a string use String::from_utf8_lossy(&variable.stdout or stderr)
//For status code just use variable.status

pub fn execute(command: &str) -> Output{
    //Check for input before using execute as nothing but spaces crashes
    //trim() before input
    let vec: Vec<&str> = command.split(" ").collect();
    let args = vec.as_slice();
        let output = if args.len() > 1 {
            Command::new(&args[0]).args(&args[1.. ]).output()
                .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })
        } else if args.len() == 1{
            Command::new(&args[0]).output()
                .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })
        } else {
            Command::new("").output()
                .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) })};
    println!("{}",String::from_utf8_lossy(&output.stdout));
    output
 }

//pub fn pipe(){};


#[cfg(test)]
mod tests{
    use std::process::*;
    use super::*;

    #[test]
    fn test_execute(){
        let test = execute("echo hello there");
        let output = Command::new("echo")
                .arg("hello there")
                .output()
                .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    //Test for Error, Exit Code and Output equality
        assert_eq!(test.status, output.status);
        assert_eq!(String::from_utf8_lossy(&test.stdout),String::from_utf8_lossy(&output.stdout));
        assert_eq!(String::from_utf8_lossy(&test.stderr),String::from_utf8_lossy(&output.stderr));

    }
    #[test]
    #[should_panic]
    fn test_execute_fail(){
        let test = execute("echo hello");
        let output = Command::new("/bin/ls")
                .arg("-al")
                .output()
                .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    //Test for Error, Exit Code and Output equality
        assert_eq!(test.status, output.status);
        assert_eq!(String::from_utf8_lossy(&test.stdout),String::from_utf8_lossy(&output.stdout));
        assert_eq!(String::from_utf8_lossy(&test.stderr),String::from_utf8_lossy(&output.stderr));

    }
}
