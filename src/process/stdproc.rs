use std::process::Output;

///Get Stdout or Err
///Returns the standard output or error of an executed command or returns that
///the command was invalid
pub fn get_stdout_or_stderr(output: Option<Output>) -> String {
    if output.is_some() {
        let temp = output.expect("Output has been checked");
        if temp.stdout.is_empty() {
            String::from_utf8(temp.stderr)
                .expect("Should have translated to string easily")
        } else {
            String::from_utf8(temp.stdout)
                .expect("Should have translated to string easily")
        }
    } else {
        "Please input a valid command".to_owned()
    }
}


pub fn get_status(output: Option<Output>) -> bool {
    if output.is_some() {
        let temp = output.expect("Output has been checked");
        temp.status.success()
    } else {
        false
    }
}

