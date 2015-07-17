use core::autocomplete;
use core::keybinding;
use core::history::History;

pub struct Input_Buffer<'a> {
    //We want this to last the whole time rusty is running so 'static is an appropriate lifetime
    //here. Also gets the compiler to shut the hell up
    parsed: Vec<&'a str>,//Output vector to use in interpretation of whatever need be
    hist: History<'a>, //History struct containing old parsed vectors
}

//Flush parsed buffer if need be if keyboard interupts are recieved
//etc etc etc

impl Input_Buffer<'static> {

    pub fn new() -> Input_Buffer<'static> {
        Input_Buffer {
            parsed: Vec::new(),
            hist: History::new(),
        }
    }

    pub fn get_parsed(&self) -> &Vec<&str> {
        unimplemented!()
    }
    pub fn get_mut_parsed(&mut self) -> &mut Vec<&str> {
        unimplemented!()
    }
    pub fn get_hist(&self) -> &History {
        unimplemented!()
    }
    pub fn get_mut_hist(&mut self) -> &mut History {
        unimplemented!()
    }
}
