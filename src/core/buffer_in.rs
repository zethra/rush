extern crate libc;
use std::io::{stdout,Write};
use core::keybinding::*;

extern {
   fn get_input() -> libc::c_int;
   fn backspace(input: libc::c_int);
   fn right(input: libc::c_int);
   fn left(input: libc::c_int);
}

pub struct InputBuffer {
    pub line: String,
}

impl InputBuffer {

    pub fn new() -> Self {
        let mut _buffer = String::new();
        InputBuffer {
            line: _buffer,
        }
    }

    //Rads key strokes into buffer. If a certain key is recieved
    //activates various commands
    #[allow(unreachable_code)]
    pub fn readline(&mut self) -> Key {
        let mut line = String::new();
        let mut ch;
        let mut bol = true;
        loop {
            ch  = unsafe {get_input()};
            let keypress = new_key(ch);
            match keypress {
                Key::Enter => {
                    println!("");
                    stdout().flush().ok().expect("Could not flush stdout");
                    self.line = line;
                    return Key::Null;
                }
                Key::Char(c) => {
                    bol = false;
                    line.push(c);
                    print!("{}",c);
                    stdout().flush().ok().expect("Could not flush stdout");
                }
                Key::Left => {
                    unsafe{
                        left(1);
                    }
                }
                Key::Right => {
                    unsafe{
                        right(1);
                    }
                }
                Key::Backspace => {
                    line.pop();
                    if !line.is_empty(){
                        unsafe{backspace(0);}
                    } else if !bol {
                        unsafe{backspace(1);}
                        bol = true;
                    } else if bol {
                        unsafe{backspace(2);}
                    }
                },
                _ => {
                    println!(""); //Remove once all keys are implemented
                    stdout().flush().ok().expect("Could not flush stdout");
                    self.line = line;
                    return keypress;
                }
            }
        }
        unreachable!()
    }

    //Outputs buffer for usage puts line into history
    pub fn output(&mut self) -> Vec<&str> {
        let out_vec: Vec<&str> = self.line.trim().split(' ').collect();
        out_vec
    }

}
