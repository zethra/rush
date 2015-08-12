use std::env::{current_dir, home_dir};
use core::config::read_config_prompt;
use std::io::{stdout, Write};

///Prompt
///Struct containing prompt and cwd for use on every new line
///in Rusty
pub struct Prompt {

    user_p: String,
    cwd: String,

}

impl Prompt {

    ///Instantiates a new Prompt with default values
    ///that will be overwritten when the configuration is updated
    ///in the main file for execution
    pub fn new() -> Prompt {
        let mut object = Prompt {
            user_p: "michael@flame %".to_owned(),
            cwd: "~/".to_owned(),
        };
        object.update_cwd();
        object.update_prompt();
        object
    }

    ///Update Prompt
    ///Calls method in rush::config to update the current prompt
    ///Only needs to be called if using cd or su at this point
    ///in time
    pub fn update_prompt(&mut self) {
        self.user_p = read_config_prompt(&self);
    }

    ///Get User P
    ///Returns prompt to be displayed on the command line
    pub fn get_user_p(&self) -> String {
        self.user_p.to_owned()
    }

    ///Get CWD
    ///Returns the CWD for use in prompts
    pub fn get_cwd(&self) -> String {
        self.cwd.to_owned()
    }

    ///Update CWD
    ///Used to update the CWD if using CD
    pub fn update_cwd(&mut self){
        let buff = current_dir().ok().expect("No current directory");

        //Makes cwd ~/ if in home directory of user otherwise
        //just the current directory
        if buff.starts_with(home_dir().expect("No Home directory").as_path()){
        let mut home = "~/".to_owned();
            home.push_str(buff.as_path().relative_from(home_dir()
                                        .expect("No Home directory")
                                        .as_path()
                                        )
                .expect("Couldn't get relative path")
                .to_str().expect("Failed to become a str"));
            self.cwd = home;
        } else {
            self.cwd = buff.as_path()
                .to_str().expect("Failed to turn path into str").to_owned();
        }

    }

    ///Print
    ///Outputs the prompt to stdout
    pub fn print(&self) {
        print!("{} ", self.get_user_p());
        stdout().flush().ok().expect("Failed to put prompt on line");
    }

}

#[cfg(test)]
mod tests{

    #[allow(unused_imports)]
    use std::env::{current_dir,home_dir};
    use super::*;

    #[test]
    fn prompt_init() {
        let testp = Prompt::new();
        assert_eq!(testp.get_user_p(),"michael@flame %".to_owned());
        assert_eq!(testp.get_cwd(),"~/".to_owned());
    }

    #[test]
    fn updated_cwd() {
        let mut testp = Prompt::new();
        testp.update_cwd();
        assert_eq!(testp.get_cwd(), current_dir().ok()
                   .expect("Couldn't get current directory").as_path()
                   .to_str()
                   .expect("Failed to go to string").to_owned());
    }
}
