
/home/michael/Code/Rust/rusty/build.rs,44
fn main() {main3,19
fn main() {main3,19

/home/michael/Code/Rust/rusty/src/scripting/script_macros.rs,56
macro_rules! run {run2,16
macro_rules! run {run2,16

/home/michael/Code/Rust/rusty/src/scripting/mod.rs,82
pub mod script_macros;script_macros1,0
pub mod script_macros;script_macros1,0

/home/michael/Code/Rust/rusty/src/lib.rs,176
pub mod utils;utils8,163
pub mod utils;utils8,163
pub mod core;core9,178
pub mod core;core9,178
pub mod scripting;scripting10,192
pub mod scripting;scripting10,192

/home/michael/Code/Rust/rusty/src/utils/cd.rs,464
pub fn change_directory(input: Vec<&str>){change_directory6,212
pub fn change_directory(input: Vec<&str>){change_directory6,212
mod tests {tests46,1420
mod tests {tests46,1420
    fn test_change_directory_ok(){test_change_directory_ok53,1530
    fn test_change_directory_ok(){test_change_directory_ok53,1530
    fn test_change_directory_fail(){test_change_directory_fail63,1816
    fn test_change_directory_fail(){test_change_directory_fail63,1816

/home/michael/Code/Rust/rusty/src/utils/calc.rs,578
pub fn calculate() {calculate4,44
pub fn calculate() {calculate4,44
fn add(num1: i64, num2: i64) -> i64 {add10,269
fn add(num1: i64, num2: i64) -> i64 {add10,269
fn mult(num1: i64, num2: i64) -> i64 {mult14,326
fn mult(num1: i64, num2: i64) -> i64 {mult14,326
fn div(num1: i64, num2: i64) -> i64 {div18,384
fn div(num1: i64, num2: i64) -> i64 {div18,384
fn sub(num1: i64, num2: i64) -> i64 {sub22,441
fn sub(num1: i64, num2: i64) -> i64 {sub22,441
fn modulo(num1: i64, num2: i64) -> i64 {modulo26,498
fn modulo(num1: i64, num2: i64) -> i64 {modulo26,498

/home/michael/Code/Rust/rusty/src/utils/cat.rs,56
pub fn concat(){concat1,0
pub fn concat(){concat1,0

/home/michael/Code/Rust/rusty/src/utils/mod.rs,130
pub mod cd;cd1,0
pub mod cd;cd1,0
pub mod calc;calc2,12
pub mod calc;calc2,12
pub mod cat;cat3,26
pub mod cat;cat3,26

/home/michael/Code/Rust/rusty/src/main.rs,48
fn main() {main14,325
fn main() {main14,325

/home/michael/Code/Rust/rusty/src/core/autocomplete.rs,0

/home/michael/Code/Rust/rusty/src/core/error.rs,676
pub enum InterpretError {InterpretError5,48
pub enum InterpretError {InterpretError5,48
pub impl From<io::Error> for InterpretError {From for InterpretError10,127
pub impl From<io::Error> for InterpretError {From for InterpretError10,127
    fn from(err: io::Error) -> InterpretError {from11,173
    fn from(err: io::Error) -> InterpretError {from11,173
pub impl From<string::ParseError> for InterpretError {From for InterpretError16,262
pub impl From<string::ParseError> for InterpretError {From for InterpretError16,262
    fn from(err: string::ParseError) -> InterpretError {from17,317
    fn from(err: string::ParseError) -> InterpretError {from17,317

/home/michael/Code/Rust/rusty/src/core/config.rs,758
fn read_in_config() -> String{read_in_config8,161
fn read_in_config() -> String{read_in_config8,161
pub fn read_config_prompt(input: &Prompt) -> String {read_config_prompt26,917
pub fn read_config_prompt(input: &Prompt) -> String {read_config_prompt26,917
pub fn check_alias(input: Vec<&str>) -> Option<String> {check_alias66,2388
pub fn check_alias(input: Vec<&str>) -> Option<String> {check_alias66,2388
pub fn set_env_var() {set_env_var94,3268
pub fn set_env_var() {set_env_var94,3268
fn env_parse(input: String) -> String {env_parse109,3963
fn env_parse(input: String) -> String {env_parse109,3963
mod tests{tests148,5187
mod tests{tests148,5187
    fn readconfig() {readconfig152,5229
    fn readconfig() {readconfig152,5229

/home/michael/Code/Rust/rusty/src/core/keybinding.rs,148
pub enum Key {Key1,0
pub enum Key {Key1,0
pub fn new_key(input: i32) -> Key {new_key16,265
pub fn new_key(input: i32) -> Key {new_key16,265

/home/michael/Code/Rust/rusty/src/core/history.rs,362
pub struct HistoryBuffer {HistoryBuffer2,22
pub struct HistoryBuffer {HistoryBuffer2,22
impl HistoryBuffer {HistoryBuffer6,82
impl HistoryBuffer {HistoryBuffer6,82
    pub fn new() -> Self {new7,103
    pub fn new() -> Self {new7,103
    pub fn store(&mut self, input: String) {store14,244
    pub fn store(&mut self, input: String) {store14,244

/home/michael/Code/Rust/rusty/src/core/buffer_in.rs,566
   fn get_input() -> libc::c_int;get_input6,83
   fn get_input() -> libc::c_int;get_input6,83
pub struct InputBuffer {InputBuffer9,120
pub struct InputBuffer {InputBuffer9,120
impl InputBuffer {InputBuffer13,170
impl InputBuffer {InputBuffer13,170
    pub fn new() -> Self {new15,190
    pub fn new() -> Self {new15,190
    pub fn readline(&mut self) -> Key {readline24,422
    pub fn readline(&mut self) -> Key {readline24,422
    pub fn output(&mut self) -> Vec<&str> {output46,1068
    pub fn output(&mut self) -> Vec<&str> {output46,1068

/home/michael/Code/Rust/rusty/src/core/execute.rs,1766
pub fn interpret(command: Vec<&str>) -> String {interpret7,117
pub fn interpret(command: Vec<&str>) -> String {interpret7,117
fn execute(command: Vec<&str>) -> Option<Output>{execute33,870
fn execute(command: Vec<&str>) -> Option<Output>{execute33,870
fn get_stdout(output: Option<Output>) -> String{get_stdout45,1233
fn get_stdout(output: Option<Output>) -> String{get_stdout45,1233
fn get_stderr(output: Option<Output>) -> String{get_stderr58,1663
fn get_stderr(output: Option<Output>) -> String{get_stderr58,1663
fn get_status(output: Option<Output>) -> bool{get_status69,1994
fn get_status(output: Option<Output>) -> bool{get_status69,1994
fn split_pipes(input: Vec<&str>) -> Vec<Vec<&str>> {split_pipes79,2213
fn split_pipes(input: Vec<&str>) -> Vec<Vec<&str>> {split_pipes79,2213
fn piped(input: Vec<&str>) -> String {piped104,2919
fn piped(input: Vec<&str>) -> String {piped104,2919
fn first_pipe(command: Vec<&str>) -> Result<Child> {first_pipe130,3705
fn first_pipe(command: Vec<&str>) -> Result<Child> {first_pipe130,3705
fn execute_pipe(command: Vec<&str>, child: Child) -> Result<Child> {execute_pipe147,4170
fn execute_pipe(command: Vec<&str>, child: Child) -> Result<Child> {execute_pipe147,4170
fn final_pipe(command: Vec<&str>, child: Child) -> String {final_pipe171,4961
fn final_pipe(command: Vec<&str>, child: Child) -> String {final_pipe171,4961
mod tests{tests201,5930
mod tests{tests201,5930
    fn pipes() {pipes205,5972
    fn pipes() {pipes205,5972
    fn pipes_fail() {pipes_fail214,6204
    fn pipes_fail() {pipes_fail214,6204
    fn execute(){execute222,6436
    fn execute(){execute222,6436
    fn execute_fail(){execute_fail231,6628
    fn execute_fail(){execute_fail231,6628

/home/michael/Code/Rust/rusty/src/core/logic.rs,0

/home/michael/Code/Rust/rusty/src/core/prompt.rs,870
pub struct Prompt {Prompt7,298
pub struct Prompt {Prompt7,298
impl Prompt {Prompt14,360
impl Prompt {Prompt14,360
    pub fn new() -> Prompt {new16,375
    pub fn new() -> Prompt {new16,375
    pub fn update_prompt(&mut self) {update_prompt23,522
    pub fn update_prompt(&mut self) {update_prompt23,522
    pub fn get_user_p(&self) -> String {get_user_p27,616
    pub fn get_user_p(&self) -> String {get_user_p27,616
    pub fn get_cwd(&self) -> String {get_cwd31,695
    pub fn get_cwd(&self) -> String {get_cwd31,695
    pub fn update_cwd(&mut self){update_cwd35,768
    pub fn update_cwd(&mut self){update_cwd35,768
mod tests{tests57,1486
mod tests{tests57,1486
    fn prompt_init() {prompt_init63,1599
    fn prompt_init() {prompt_init63,1599
    fn updated_cwd() {updated_cwd70,1800
    fn updated_cwd() {updated_cwd70,1800

/home/michael/Code/Rust/rusty/src/core/mod.rs,506
pub mod execute;execute1,0
pub mod execute;execute1,0
pub mod autocomplete;autocomplete2,17
pub mod autocomplete;autocomplete2,17
pub mod logic;logic3,39
pub mod logic;logic3,39
pub mod prompt;prompt4,54
pub mod prompt;prompt4,54
pub mod config;config5,70
pub mod config;config5,70
pub mod keybinding;keybinding6,86
pub mod keybinding;keybinding6,86
pub mod buffer_in;buffer_in7,106
pub mod buffer_in;buffer_in7,106
pub mod history;history8,125
pub mod history;history8,125
/home/michael/.rusty-tags/cache/libc-0.1.8.emacs,include
/home/michael/.rusty-tags/cache/toml-0.1.21.emacs,include
