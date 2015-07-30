use std::env::{current_dir, home_dir};
use core::config::read_config_prompt;
//For now it will access the toml file in the config directory
//Later give it the means to access it from ~/.rusty.toml
//Activate this only when performing su or cd that way it reduces
//need to keep getting the prompt
pub struct Prompt {

    user_p: String,
    cwd: String,

}

impl Prompt {

    pub fn new() -> Prompt {
        Prompt {
            user_p: "michael@flame %".to_string(),
            cwd: "~/".to_string(),
        }
    }

    pub fn update_prompt(&mut self) {
        self.user_p = read_config_prompt(&self);
    }

    pub fn get_user_p(&self) -> String {
        self.user_p.to_owned()
    }

    pub fn get_cwd(&self) -> String {
        self.cwd.to_owned()
    }

    pub fn update_cwd(&mut self){
        let buff = current_dir().ok().unwrap();

        //Makes cwd ~/ if in home directory of user otherwise
        //just the current directory
        if buff.starts_with(home_dir().unwrap().as_path()){
        let mut home = "~/".to_string();
            home.push_str(buff.as_path().relative_from(home_dir()
                                                   .unwrap()
                                                   .as_path()
                                                   )
                .unwrap().to_str().unwrap());
            self.cwd = home;
        } else {
            self.cwd = buff.as_path().to_str().unwrap().to_string();
        }

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
        assert_eq!(testp.get_user_p(),"michael@flame %".to_string());
        assert_eq!(testp.get_cwd(),"~/".to_string());
    }

    /*#[test]
    fn updated_cwd() {
        let mut testp = Prompt::new();
        testp.update_cwd();
        assert_eq!(testp.get_cwd(),current_dir().ok().unwrap().as_path().to_str().unwrap().to_string());
    }
    */
}
