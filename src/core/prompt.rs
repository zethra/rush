extern crate toml;
use std::env::{var, current_dir, home_dir};
use std::fs::File;
use std::process::Command;
use std::io::{Read, BufReader};

//For now it will access the toml file in the config directory
//Later give it the means to access it from ~/.rusty.toml
//Activate this only when performing su or cd that way it reduces
//need to keep getting the prompt
pub struct Prompt {
    
    user_p: String,
    cwd: String,
    
}

fn read_config_prompt(input: &Prompt) -> String {
    let config = match File::open("/home/michael/Code/Rust/rusty/config/rusty.toml") {
        Ok(config) => config,
        Err(..) => panic!("Config non existent"),
    };
   
    //Read toml file and put into parser
    let mut reader = BufReader::new(&config);
    let buffer_string = &mut String::new();
    reader.read_to_string(buffer_string);
     
    let value: toml::Value = buffer_string.parse().unwrap();
    let left = value.lookup("prompt.left").unwrap().as_str().unwrap().split("%");
    let mut prompt = "".to_string();
    for i in left {
        if i.len() > 0 {
            match i.char_at(0) {
                'U' => prompt.push_str(&var("USER").ok().unwrap()),
                'H' => prompt.push_str(&String::from_utf8(Command::new("uname").arg("-n").output().ok().unwrap().stdout).unwrap().trim()),
                'L' => prompt.push_str(&input.get_cwd()),
                'R' => {
                    let uid = String::from_utf8(Command::new("uname").arg("-n").output().ok().unwrap().stdout).ok().unwrap();
                    if uid == "0" {
                        prompt.push('#');
                    } else {
                        prompt.push('%');
                    }

                }
                 _ => prompt.push(i.char_at(0)),
            }
        }
        //Add non Prompt special chars to prompt
        if i.len() > 1 {
            for j in 1 .. i.len() {
                prompt.push(i.char_at(j));
            }
        }
    }

    prompt
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
