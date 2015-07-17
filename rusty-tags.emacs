
/home/michael/Code/Rust/rusty/src/scripting/script_macros.rs,28
macro_rules! run {run2,16

/home/michael/Code/Rust/rusty/src/scripting/mod.rs,41
pub mod script_macros;script_macros1,0

/home/michael/Code/Rust/rusty/src/lib.rs,88
pub mod utils;utils8,165
pub mod core;core9,180
pub mod scripting;scripting10,194

/home/michael/Code/Rust/rusty/src/utils/cd.rs,232
pub fn change_directory(input: Vec<&str>){change_directory6,212
mod tests {tests46,1420
    fn test_change_directory_ok(){test_change_directory_ok53,1530
    fn test_change_directory_fail(){test_change_directory_fail63,1816

/home/michael/Code/Rust/rusty/src/utils/calc.rs,289
pub fn calculate() {calculate4,44
fn add(num1: i64, num2: i64) -> i64 {add10,269
fn mult(num1: i64, num2: i64) -> i64 {mult14,326
fn div(num1: i64, num2: i64) -> i64 {div18,384
fn sub(num1: i64, num2: i64) -> i64 {sub22,441
fn modulo(num1: i64, num2: i64) -> i64 {modulo26,498

/home/michael/Code/Rust/rusty/src/utils/cat.rs,28
pub fn concat(){concat1,0

/home/michael/Code/Rust/rusty/src/utils/mod.rs,65
pub mod cd;cd1,0
pub mod calc;calc2,12
pub mod cat;cat3,26

/home/michael/Code/Rust/rusty/src/main.rs,24
fn main() {main11,268

/home/michael/Code/Rust/rusty/src/core/autocomplete.rs,0

/home/michael/Code/Rust/rusty/src/core/error.rs,338
pub enum InterpretError {InterpretError5,48
pub impl From<io::Error> for InterpretError {From for InterpretError10,127
    fn from(err: io::Error) -> InterpretError {from11,173
pub impl From<string::ParseError> for InterpretError {From for InterpretError16,262
    fn from(err: string::ParseError) -> InterpretError {from17,317

/home/michael/Code/Rust/rusty/src/core/config.rs,379
fn read_in_config() -> String{read_in_config8,161
pub fn read_config_prompt(input: &Prompt) -> String {read_config_prompt26,917
pub fn check_alias(input: Vec<&str>) -> Option<String> {check_alias66,2389
pub fn set_env_var() {set_env_var94,3269
fn env_parse(input: String) -> String {env_parse109,3964
mod tests{tests148,5188
    fn readconfig() {readconfig152,5230

/home/michael/Code/Rust/rusty/src/core/keybinding.rs,0

/home/michael/Code/Rust/rusty/src/core/history.rs,142
pub struct History {History1,0
impl History {History7,87
    pub fn new() -> History {new8,102
    fn cap_it(&mut self) {cap_it13,276

/home/michael/Code/Rust/rusty/src/core/buffer_in.rs,402
pub struct Input_Buffer {Input_Buffer5,66
impl Input_buffer {Input_buffer13,327
    pub fn new() -> Input_Buffer {new15,348
    pub fn get_parsed(&self) -> &Vec<&str> {get_parsed22,489
    pub fn get_mut_parsed(&mut self) -> &mut Vec<&str> {get_mut_parsed25,565
    pub fn get_hist(&self) -> &History {get_hist28,653
    pub fn get_mut_hist(&mut self) -> &mut History {get_mut_hist31,725

/home/michael/Code/Rust/rusty/src/core/execute.rs,883
pub fn interpret(command: Vec<&str>) -> String {interpret7,117
fn execute(command: Vec<&str>) -> Option<Output>{execute33,870
fn get_stdout(output: Option<Output>) -> String{get_stdout45,1233
fn get_stderr(output: Option<Output>) -> String{get_stderr58,1663
fn get_status(output: Option<Output>) -> bool{get_status69,1994
fn split_pipes(input: Vec<&str>) -> Vec<Vec<&str>> {split_pipes79,2213
fn piped(input: Vec<&str>) -> String {piped104,2919
fn first_pipe(command: Vec<&str>) -> Result<Child> {first_pipe130,3705
fn execute_pipe(command: Vec<&str>, child: Child) -> Result<Child> {execute_pipe147,4170
fn final_pipe(command: Vec<&str>, child: Child) -> String {final_pipe171,4961
mod tests{tests201,5930
    fn pipes() {pipes205,5972
    fn pipes_fail() {pipes_fail214,6204
    fn execute(){execute222,6436
    fn execute_fail(){execute_fail231,6628

/home/michael/Code/Rust/rusty/src/core/logic.rs,0

/home/michael/Code/Rust/rusty/src/core/prompt.rs,435
pub struct Prompt {Prompt7,298
impl Prompt {Prompt14,360
    pub fn new() -> Prompt {new16,375
    pub fn update_prompt(&mut self) {update_prompt23,524
    pub fn get_user_p(&self) -> String {get_user_p27,618
    pub fn get_cwd(&self) -> String {get_cwd31,697
    pub fn update_cwd(&mut self){update_cwd35,770
mod tests{tests57,1490
    fn prompt_init() {prompt_init62,1574
    fn updated_cwd() {updated_cwd69,1777

/home/michael/Code/Rust/rusty/src/core/mod.rs,253
pub mod execute;execute1,0
pub mod autocomplete;autocomplete2,17
pub mod logic;logic3,39
pub mod prompt;prompt4,54
pub mod config;config5,70
pub mod keybinding;keybinding6,86
pub mod buffer_in;buffer_in7,106
pub mod history;history8,125
/home/michael/.rusty-tags/cache/term-0.2.10.emacs,include
/home/michael/.rusty-tags/cache/toml-0.1.21.emacs,include
