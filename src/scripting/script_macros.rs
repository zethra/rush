#[macro_export]
macro_rules! run {
    ($command:expr) => {
        {
            use std::process::Command;
            let output = Command::new($command).output().ok();
            match output.is_some(){
                true => {
                    let temp = output.unwrap();
                    println!("{}",String::from_utf8(temp.stdout).unwrap());
                },
                false => println!("Please input a valid command")
            }
        }
    };
    ($command:expr,$args:expr) => {
        {
            use std::process::Command;
            let output = Command::new($command).args($args).output().ok();
            match output.is_some(){
                true => {
                    let temp = output.unwrap();
                    println!("{}",String::from_utf8(temp.stdout).unwrap());
                },
                false => println!("Please input a valid command")
            }
        }
    };
}
