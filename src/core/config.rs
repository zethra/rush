extern crate toml;
use std::io::{Read,BufReader};
use std::fs::File;
use std::fmt;
use std::env::var;
use std::process::Command;
use core::prompt::Prompt;

fn read_in_config() -> String{
    let config = match
    File::open("/home/michael/Code/Rust/rusty/config/rusty.toml") {
        Ok(config) => config,
        Err(..) => panic!("Config non existent"),
    };

    //Read toml file and put into parser
    let mut reader = BufReader::new(&config);
    let mut buffer_string = String::new();
    reader.read_to_string(&mut buffer_string)
        .ok().expect("Failed to read in config"); buffer_string
}

pub fn read_config_prompt(input: &Prompt) -> String {
    let buffer_string = read_in_config();

    let value: toml::Value = buffer_string.parse().unwrap();
    let left = value.lookup("prompt.left").unwrap().as_str()
        .unwrap().split("%");
    let mut prompt = "".to_string();
    for i in left {
        if i.len() > 0 {
            match i.char_at(0) {
                'U' => prompt.push_str(&var("USER").ok().unwrap()),
                'H' => prompt.push_str(&String::from_utf8(Command::new("uname")
                                                          .arg("-n").output()
                                                          .ok().unwrap().stdout)
                                       .unwrap().trim()),
                'L' => prompt.push_str(&input.get_cwd()),
                'R' => {
                    let uid = String::from_utf8(Command::new("uname").arg("-n")
                                                .output().ok().unwrap().stdout)
                        .ok().unwrap();
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

pub fn check_alias(input: Vec<&str>) {//-> Option<Vec<&str>> {
    //Checks if alias is in config file and returns the altered
    //version as an Option of the input. If succesfully found
    //it can be unwraped for execution

}

