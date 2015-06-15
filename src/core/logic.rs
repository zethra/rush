//Logic interpretation
use core::execute::{execute, get_status, get_stdout, get_stderr};

pub fn and(command1:Vec<&str>, command2:Vec<&str>){
//&&
//Run the first command. If it passes run the second one. Otherwise output failure and don't run
//the second one

    let command1_exec = execute(command1);
    let command1_status = get_status(command1_exec.clone());
    
    if command1_status {
        
        println!("{}", get_stdout(command1_exec.clone()));
        
        let command2_exec = execute(command2.clone());
        let command2_status = get_status(command2_exec.clone());
        
        if command2_status {
            println!("{}", get_stdout(command2_exec.clone()));
        } else {
            println!("{}", get_stderr(command2_exec.clone()));
        }
    
    } else {
        println!("{}", get_stderr(command1_exec.clone()));
    }
}

pub fn or(command1:Vec<&str>, command2:Vec<&str>){
//||
//If the prior command fails execute the next one after dumping out
//stderr. Otherwise just the first command is executed.
    
    let command1_exec = execute(command1);
    let command1_status = get_status(command1_exec.clone());
    
    if command1_status {
        println!("{}", get_stdout(command1_exec.clone()));
    } else {
        
        println!("{}", get_stderr(command1_exec.clone()));
        
        let command2_exec = execute(command2.clone());
        let command2_status = get_status(command2_exec.clone());
        
        if command2_status {
            println!("{}", get_stdout(command2_exec.clone()));
        } else {
            println!("{}", get_stderr(command2_exec.clone()));
        }

    }
}

pub fn xor(command1:Vec<&str>, command2:Vec<&str>){

}

pub fn background(){
//&
//Run the command in the background and begin getting the next input and command to run
}
