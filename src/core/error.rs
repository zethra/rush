use std::io;
use std::string;

#[derive(debug)]
pub enum InterpretError {
    Io(io::Error),
    Parse(string::ParseError),
}

pub impl From<io::Error> for InterpretError {
    fn from(err: io::Error) -> InterpretError {
        InterpretError::Io(err)
    }
}

pub impl From<string::ParseError> for InterpretError {
    fn from(err: string::ParseError) -> InterpretError {
        InterpretError::Parse(err)
    }
}
