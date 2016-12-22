#![feature(plugin)]
#![feature(custom_derive)]
#![feature(stmt_expr_attributes)]
#![feature(process_exec)]
//#![plugin(clippy)]
#![doc(html_root_url = "https://mgattozzi.github.io/rusty/")]
pub mod builtins;
pub mod scripting;
pub mod process;
pub mod autocomplete;
pub mod prompt;
pub mod config;
