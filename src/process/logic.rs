#![allow(unused_imports)] //for some of the things in this file for now
#![allow(dead_code)]

use process::stdproc::{get_stdout_or_stderr, get_status};
use process::execute::interpret;

//false if it failed true if it didn't
fn and(command1: String, command2: String) -> bool {
    if interpret(command1) {
        interpret(command2);
        return true;
    }
    false
}

fn or(command1:String, command2: String) -> bool {
    if interpret(command1) {
        true
    } else {
        interpret(command2);
        false
    }
}

fn xor(command1: String, command2: String) -> bool {
    let status1 = interpret(command1);
    let status2 = interpret(command2);
    if !status1 && status2 || status1 && !status2 {
        return true;
    }
    false
}

// //Not Implemented
// fn nand(command1: Vec<&str>, command2: Vec<&str>) -> bool {
//     false
// }

#[cfg(test)]
mod tests {
    use super::{and, xor};

    #[test]
    fn and_test() {
        //Both pass
        let command1 = "ls /";
        let command2 = "ls /";
        assert_eq!(true, and(command1,command2));

        //First Fails
        let command3 = "ls /blah";
        let command4 = "ls /";
        assert_eq!(false, and(command3,command4));

        //Last Fails
        let command5 = "ls /";
        let command6 = "ls /blah";
        assert_eq!(true, and(command5,command6));

        //Both fail
        let command7 = "ls /blah";
        let command8 = "ls /blah";
        assert_eq!(false, and(command7,command8));
    }

    #[test]
    fn xor_test() {
        //Both pass
        let command1 = "ls /";
        let command2 = "ls /";
        assert_eq!(false, xor(command1,command2));

        //First Fails
        let command3 = "ls /blah";
        let command4 = "ls /";
        assert_eq!(true, xor(command3,command4));

        //Last Fails
        let command5 = "ls /";
        let command6 = "ls /blah";
        assert_eq!(true, xor(command5,command6));

        //Both fail
        let command7 = "ls /blah";
        let command8 = "ls /blah";
        assert_eq!(false, xor(command7,command8));
    }
}
