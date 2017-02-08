use std::env::home_dir;

include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

pub fn get_home_dir() -> String {
    let home_config = home_dir().expect("No Home directory");
    home_config.as_path()
        .to_str()
        .expect("Should have a home directory to turn into a str")
        .to_string()
}

#[derive(Debug)]
pub enum StackItem {
    Statement {
        statement: Command,
        next: Option<(String, Box<Statement>)>,
    },
}

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub pipe: Option<Box<Command>>,
    pub redirect: Option<Redirect>,
    pub vars: Vec<(String, Option<String>)>,
}

#[derive(Debug)]
pub struct Statement {
    pub statement: Command,
    pub next: Option<(String, Box<Statement>)>,
}

#[derive(Debug)]
pub enum Redirect {
    Fd(Option<i32>, String, String),
    DuplicateFd(Option<i32>, String, i32),
    MoveFd(Option<i32>, String, i32),
}