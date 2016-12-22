pub trait BuiltIn {
    fn run(command: Vec<String>) -> bool;
    fn run_detached(command: Vec<String>) -> bool;
    fn redirect_out(command: Vec<String>) -> bool;
    fn redirect_out_detached(command: Vec<String>) -> bool;
    fn piped(input: Vec<String>) -> bool;
    fn piped_detached(input: Vec<String>) -> bool;
    fn piped_redirect_out(input: Vec<String>) -> bool;
    fn piped_redirect_out_detached(input: Vec<String>) -> bool;
}