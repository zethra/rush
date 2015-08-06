extern crate libc;
use std::io::{stdout,Write};
use std::ffi::CString;
use core::keybinding::*;

extern {
   fn get_input() -> libc::c_int;
   fn backspace(input: libc::c_int);
   fn right(input: libc::c_int);
   fn left(input: libc::c_int);
   fn go_back(slice: *const libc::c_char,length: libc::c_int);
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

        //Line and charachter buffers
        let mut line = String::new();
        let mut ch;

        //Variables regarding the terminal cursor
        let cursor_pos_min = 0;
        let mut cursor = cursor_pos_min;
        let mut cursor_pos_max = cursor_pos_min;
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
                    if cursor == cursor_pos_max {
                        line.push(c);
                        cursor += 1;
                        cursor_pos_max += 1;
                        print!("{}",c);
                        stdout().flush().ok()
                            .expect("Could not flush stdout");
                    }
                }
                Key::Left => {
                    if cursor > cursor_pos_min {
                        unsafe{
                            left(1);
                        }
                        cursor -=1;
                    }
                }
                Key::Right => {
                    if cursor < cursor_pos_max {
                        unsafe{
                            right(1);
                        }
                        cursor += 1;
                    }
                }
                Key::Backspace => {
                    //Need to change c file so that the buffer is reprinted
                    //let mut temp_buffer = String::new();
                    if cursor >= cursor_pos_min + 1{
                        if !line.is_empty(){
                            line.remove(cursor - 1);
                            cursor -= 1;
                            cursor_pos_max -= 1;
                            unsafe{backspace(0);}
                            let slice = CString::new(&line[cursor..])
                                .unwrap();
                            unsafe{go_back(slice.as_ptr(),
                                           slice.as_bytes().len() as i32);}
                        }
                    } else if cursor == cursor_pos_min {
                        unsafe{backspace(1);}
                    }
                },
                Key::Tab => {
                    //Autocomplete
                    return Key::Null;
                }
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
