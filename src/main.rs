
extern crate rusty;

use rusty::utils::cd;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    cd::change_directory(Path::new(".."));
}
