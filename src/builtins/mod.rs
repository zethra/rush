use std::collections::HashMap;

mod cd;
mod bools;
mod export;
mod source;
mod exit;

pub type Builtin = fn(&Vec<String>) -> bool;

pub fn get_builtins() -> HashMap<String, Builtin> {
    let mut builtins = HashMap::new();
    builtins.insert("exit".to_string(), exit::exit as Builtin);
    builtins.insert("cd".to_string(), cd::change_directory as Builtin);
    builtins.insert("true".to_string(), bools::bool_true as Builtin);
    builtins.insert("false".to_string(), bools::bool_false as Builtin);
    builtins.insert("export".to_string(), export::export as Builtin);
    builtins.insert("source".to_string(), source::source as Builtin);
    builtins
}