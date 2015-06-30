
/home/michael/Code/Rust/rusty/src/lib.rs,52
pub mod utils;utils7,138
pub mod core;core8,153

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
fn main() {main10,211

/home/michael/Code/Rust/rusty/src/core/autocomplete.rs,0

/home/michael/Code/Rust/rusty/src/core/config.rs,216
fn read_in_config() -> String{read_in_config8,142
pub fn read_config_prompt(input: &Prompt) -> String {read_config_prompt23,603
pub fn check_alias(input: Vec<&str>) { //-> Option<Vec<&str>> {check_alias58,1840

/home/michael/Code/Rust/rusty/src/core/script.rs,0

/home/michael/Code/Rust/rusty/src/core/execute.rs,883
pub fn interpret(command: Vec<&str>) -> String {interpret7,195
fn execute(command: Vec<&str>) -> Option<Output>{execute33,952
fn get_stdout(output: Option<Output>) -> String{get_stdout45,1315
fn get_stderr(output: Option<Output>) -> String{get_stderr58,1745
fn get_status(output: Option<Output>) -> bool{get_status69,2076
fn split_pipes(input: Vec<&str>) -> Vec<Vec<&str>> {split_pipes79,2295
fn piped(input: Vec<&str>) -> String {piped104,3001
fn first_pipe(command: Vec<&str>) -> Result<Child> {first_pipe130,3788
fn execute_pipe(command: Vec<&str>, child: Child) -> Result<Child> {execute_pipe147,4253
fn final_pipe(command: Vec<&str>, child: Child) -> String {final_pipe171,5044
mod tests{tests202,6042
    fn pipes() {pipes206,6088
    fn pipes_fail() {pipes_fail215,6324
    fn execute(){execute223,6560
    fn execute_fail(){execute_fail232,6752

/home/michael/Code/Rust/rusty/src/core/logic.rs,0

/home/michael/Code/Rust/rusty/src/core/prompt.rs,324
pub struct Prompt {Prompt8,317
impl Prompt {Prompt15,387
    pub fn new() -> Prompt {new17,406
    pub fn update_prompt(&mut self) {update_prompt24,559
    pub fn get_user_p(&self) -> String {get_user_p28,653
    pub fn get_cwd(&self) -> String {get_cwd32,735
    pub fn update_cwd(&mut self){update_cwd36,812

/home/michael/Code/Rust/rusty/src/core/mod.rs,179
pub mod execute;execute1,0
pub mod script;script2,17
pub mod autocomplete;autocomplete3,33
pub mod logic;logic4,55
pub mod prompt;prompt5,70
pub mod config;config6,86
/home/michael/.rusty-tags/cache/toml-0.1.21.emacs,include
