use core::autocomplete;
use core::keybinding;
use core::history::History;

pub struct Input_Buffer<'a> {
    line: Vec<&'a str>,//Output vector to use in interpretation of whatever need be
    hist: History<'a>, //History struct containing old parsed vectors
}

//Flush parsed buffer if need be if keyboard interupts are recieved
//etc etc etc

//We want this to last the whole time rusty is running so 'static is an
//appropriate lifetime here. Also gets the compiler to shut the hell up
impl Input_Buffer {

    pub fn new<'a>() -> Input_Buffer<'a> {
        Input_Buffer {
            line: Vec::new(),
            hist: History::new(),
        }
    }

    pub fn get_line(&self) -> &Vec<&str> {
        &self.line
    }
    pub fn get_mut_line(&mut self) -> &mut Vec<&str> {
        &mut self.line
    }
    pub fn get_hist(&self) -> &History {
        &self.hist
    }
    pub fn get_mut_hist(&mut self) -> &mut History {
        &mut self.hist
    }
}
