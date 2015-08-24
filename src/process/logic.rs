#![allow(unreachable_code)]
#![allow(dead_code)]

use process::execute::{get_stdout_or_stderr, execute, get_status};

//false if it failed true if it didn't
fn and(command1: Vec<&str>, command2: Vec<&str>) -> bool {
    let executed1 = execute(command1);
    let status = get_status(executed1.clone());
    let output1 = get_stdout_or_stderr(executed1);
    println!("{}", output1);
    if status {
        println!("{}",get_stdout_or_stderr(execute(command2)));
        return true;
    }
    false
}

//Or does not output failed command
//DOES NOT WORK RIGHT for logic
fn or(command1: Vec<&str>, command2: Vec<&str>) -> bool {
    let executed1 = execute(command1);
    let status = get_status(executed1.clone());
    if !status { //If the command failed run this
        println!("{}",get_stdout_or_stderr(execute(command2)));
        true
    } else {
        println!("{}",get_stdout_or_stderr(executed1));
        false
    }
}

fn xor(command1: Vec<&str>, command2: Vec<&str>) -> bool {
    let executed1 = execute(command1);
    let status1 = get_status(executed1.clone());
    let executed2 =execute(command2);
    let status2 = get_status(executed2.clone());
    println!("{}",get_stdout_or_stderr(executed1));
    println!("{}",get_stdout_or_stderr(executed2));
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
mod tests{
    use super::{and,xor};

    #[test]
    fn and_test(){
        //Both pass
        let command1: Vec<&str> = "date".trim().split(' ').collect();
        let command2: Vec<&str> = "ls /".trim().split(' ').collect();
        assert_eq!(true, and(command1,command2));

        //First Fails
        let command3: Vec<&str> = "date %d".trim().split(' ').collect();
        let command4: Vec<&str> = "ls /".trim().split(' ').collect();
        assert_eq!(false, and(command3,command4));

        //Last Fails
        let command5: Vec<&str> = "date".trim().split(' ').collect();
        let command6: Vec<&str> = "ls /blah".trim().split(' ').collect();
        assert_eq!(true, and(command5,command6));

        //Both fail
        let command7: Vec<&str> = "ls /blah".trim().split(' ').collect();
        let command8: Vec<&str> = "date %d".trim().split(' ').collect();
        assert_eq!(false, and(command7,command8));
    }

    #[test]
    fn xor_test(){
        //Both pass
        let command1: Vec<&str> = "date".trim().split(' ').collect();
        let command2: Vec<&str> = "ls /".trim().split(' ').collect();
        assert_eq!(false, xor(command1,command2));

        //First Fails
        let command3: Vec<&str> = "date %d".trim().split(' ').collect();
        let command4: Vec<&str> = "ls /".trim().split(' ').collect();
        assert_eq!(true, xor(command3,command4));

        //Last Fails
        let command5: Vec<&str> = "date".trim().split(' ').collect();
        let command6: Vec<&str> = "ls /blah".trim().split(' ').collect();
        assert_eq!(true, xor(command5,command6));

        //Both fail
        let command7: Vec<&str> = "ls /blah".trim().split(' ').collect();
        let command8: Vec<&str> = "date %d".trim().split(' ').collect();
        assert_eq!(false, xor(command7,command8));
    }
}

