include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[derive(Debug)]
pub enum StackItem {
    Statement {statement: Command, next: Option<(String, Box<Statement>)>},
}

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub post: Vec<String>,
    pub pipe: Option<Box<Command>>,
}

#[derive(Debug)]
pub struct Statement {
    pub statement: Command,
    pub next: Option<(String, Box<Statement>)>,
}