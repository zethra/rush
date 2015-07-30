//use core::autocomplete;
//use core::keybinding;
use std::io;
use std::mem; //Better way to not use mem replace maybe?
use std::io::Read;

pub struct InputBuffer {
    pub line: String,
    pub hist: Vec<String>,
}

impl InputBuffer {

    pub fn new() -> Self {
        let mut buffer = String::new();
        let mut vector: Vec<String> = Vec::new();
        InputBuffer {
            line: buffer,
            hist: vector,
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
                if unwrapped == '\n' {
                    break;
                }
                buffer.push(unwrapped);
            }
        }
        self.line = buffer;
    }

    //Outputs buffer for usage puts line into history
    pub fn output(&mut self) -> Vec<&str> {
        let out_vec: Vec<&str> = self.line.trim().split(' ').collect();
        out_vec
    }

    pub fn store(&mut self, line: String) {
        self.hist.push(line);
        self.line = String::new();
    }

}
