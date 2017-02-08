use interpeter::*;
use builtins::get_builtins;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn source(args: &Vec<String>) -> bool {
    let f = match File::open(&args[0]) {
        Ok(f) => f,
        Err(_) => {
            println!("Couldn't open file {}", args[0]);
            return false;
        }
    };
    let file = BufReader::new(&f);
    let builtins = get_builtins();
    for line in file.lines() {
        let l = line.unwrap();
        match interpet_line(l, &builtins) {
            ReturnValue::True => {}
            ReturnValue::False => {}
            ReturnValue::Exit(v) => {
                if v == 0 {
                    return true;
                } else {
                    return false;
                }
            }
        }
    };
    true
}