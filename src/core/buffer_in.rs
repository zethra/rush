extern crate libc;
use std::io::{stdout,Write};

extern {
   fn get_input() -> libc::c_int;
}

pub struct InputBuffer {
    pub line: String,
}

impl InputBuffer {

    pub fn new() -> Self {
        let mut buffer = String::new();
        InputBuffer {
            line: buffer,
        }
    }

    //Rads key strokes into buffer. If a certain key is recieved
    //activates various commands
    pub fn readline(&mut self) -> i32 {
        let mut line = String::new();
        let mut ch;
        loop {
            ch  = unsafe {get_input()};
            match ch {
               -1 => println!("UP"),
               -2 => println!("DOWN"),
               -3 => println!("LEFT"),
               -4 => println!("RIGHT"),
               -5 => break,
                _ => {
                    line.push(ch as u8 as char);
                    print!("{}", ch as u8 as char);
                    stdout().flush().ok().expect("Could not flush stdout");
                }
            }
        }
        println!("");
        self.line = line;
        ch
    }

    //Outputs buffer for usage puts line into history
    pub fn output(&mut self) -> Vec<&str> {
        let out_vec: Vec<&str> = self.line.trim().split(' ').collect();
        out_vec
    }

}
