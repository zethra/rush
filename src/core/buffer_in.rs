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
impl <'a> Input_Buffer<'a> {

    pub fn new() -> Input_Buffer<'a> {
        Input_Buffer {
            line: Vec::new(),
            hist: History::new(),
        }
    }

    pub fn get_line(&'a self) -> &Vec<&'a str> {
        &self.line
    }
    pub fn get_mut_line(&'a mut self) -> &mut Vec<&'a str> {
        &mut self.line
    }
    pub fn get_hist(&'a self) -> &History {
        &self.hist
    }
    pub fn get_mut_hist(&'a mut self) -> &mut History {
        &mut self.hist
    }
}
