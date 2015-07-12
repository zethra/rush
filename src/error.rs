use std::io;
use std::string;

#[derive(debug)]
pub enum InterpretError {
    Io(io::Error),
}
