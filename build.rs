extern crate gcc;

fn main() {
    gcc::Config::new().file("src/external/readline.c").compile("libreadline.a");
}
