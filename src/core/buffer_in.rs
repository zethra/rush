#![allow(unused_inports)]
use core::autocomplete;
use core::keybinding;

pub struct InputBuffer<'a> {
    pub line: Vec<&'a str>,//Output vector to use in interpretation of whatever need be
}

//Flush parsed buffer if need be if keyboard interupts are recieved
//etc etc etc

//We want this to last the whole time rusty is running so 'static is an
//appropriate lifetime here. Also gets the compiler to shut the hell up
impl <'a> InputBuffer<'a> {

    pub fn new() -> InputBuffer<'a> {
        InputBuffer {
            line: Vec::new(),
        }
    }

}
