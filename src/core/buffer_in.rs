#![allow(unused_mut)]
//use core::autocomplete;
//use core::keybinding;
use std::io;
use std::io::Read;

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
    pub fn readline(&mut self) {
        let mut buffer = String::new();
        let mut stdin = io::stdin().chars();
        for c in stdin {
            if c.is_ok() {
                let unwrapped = c.unwrap();
                match unwrapped {
                    '\n' => break,
                      _  => buffer.push(unwrapped),
                }
            }
        }
        self.line = buffer;
    }

    //Outputs buffer for usage puts line into history
    pub fn output(&mut self) -> Vec<&str> {
        let out_vec: Vec<&str> = self.line.trim().split(' ').collect();
        out_vec
    }

}
