extern crate libc;
use std::io::{stdout,Write};
use std::ffi::CString;
use core::keybinding::*;
use core::history::HistoryBuffer;

///Functions involved with Manipulating the Terminal
///or getting input for the buffer
extern {
    ///Gets a char or key input for the buffer
    fn get_input() -> libc::c_int;
    ///Used to move the cursor back one space and depending on cursor
    ///position clear to end of line
    fn backspace(input: libc::c_int);
    ///Moves cursor right by one space
    fn right(input: libc::c_int);
    ///Moves cursor left by one space
    fn left(input: libc::c_int);
    ///Prints a given char array onto a cleared line and moves the cursor
    ///back to the spot where it started to print the chars
    fn go_back(slice: *const libc::c_char,length: libc::c_int);
    ///Clears the line from the current cursor position to the the end of line
    fn clear_to_end();
}

///InputBuffer
///Buffer used to handle and interpret key strokes for the command line
pub struct InputBuffer {
    line: String,
}

impl InputBuffer {
    ///New
    ///Instantiates new buffer for use
    pub fn new() -> Self {
        let mut _buffer = String::new();
        InputBuffer {
            line: _buffer,
        }
    }

    ///Reads in key strokes and determines what to do with the terminal
    ///and buffers
    #[allow(unreachable_code)]
    pub fn readline(&mut self, hist: &mut HistoryBuffer) -> Key {

        //Line and charachter buffers
        let mut line = String::new();
        //Eventually converted to a char after interfacing with C based
        //functions
        let mut ch: i32;

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
                    hist.store(self.line.clone());
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
                    } else if cursor < cursor_pos_max {
                        line.insert(cursor,c);
                        cursor_pos_max += 1;
                        let slice = CString::new(&line[cursor..])
                                .unwrap();
                        unsafe{
                            clear_to_end();
                            go_back(slice.as_ptr(), slice.as_bytes()
                                    .len() as i32);
                            right(1)
                        }
                        cursor += 1;
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
                Key::Up =>{
                    //Moves the cursor to the beginning of the line
                    //and clears it so the history can be put in
                    while cursor > cursor_pos_min {
                        unsafe{left(1);}
                    }
                    if cursor == cursor_pos_min {
                        unsafe{clear_to_end()}
                        let popped = hist.pop();
                        print!("{}", popped);
                        stdout().flush().ok()
                            .expect("Could not flush stdout");
                        cursor = popped.len();
                        cursor_pos_max = popped.len();
                        continue;
                    }
                },
                Key::Down =>{
                    continue;
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

    ///Output
    ///Outputs buffer into a format used for execution of commands by
    ///other parts of the codebase
    pub fn output(&mut self) -> Vec<&str> {
        let out_vec: Vec<&str> = self.line.trim().split(' ').collect();
        out_vec
    }

}
