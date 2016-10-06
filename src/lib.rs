#![feature(plugin)]
#![feature(custom_derive)]
#![feature(stmt_expr_attributes)]
#![plugin(clippy)]
#![doc(html_root_url = "https://mgattozzi.github.io/rusty/")]
pub mod utils;
pub mod scripting;
pub mod process;
pub mod autocomplete;
pub mod prompt;
pub mod config;
