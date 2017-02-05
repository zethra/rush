use std::collections::HashMap;


type BuiltIn = fn(args: &Vec<String>) -> bool;

fn getBuiltIns() -> HashMap<String, BuiltIn> {
    let mut builtins = HashMap::new();
    builtins
}