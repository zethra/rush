extern crate toml;
use std::io::{Read,BufReader};
use std::fs::File;
use std::env::{set_var,var,home_dir};
use std::process::Command;
use prompt::Prompt;

///Read in Config
///Inner function used to pull in a default configuration file for parsing
///or the customized one if it exists
fn read_in_config() -> String{
    //Find a way to read from default if this doesn't work. let a = if else?
    let mut home_config = home_dir().expect("No Home directory");
    home_config.push(".rusty.toml");
    let default = File::open(home_config.as_path().to_str()
                             .expect("Should have a home directory to
                                     turn into a str"));
    let config = if default.is_err(){
        //Should be changed to location of git repo if compiling on your own machine
        File::open("./config/rusty.toml").ok().expect("No default file")
        } else {
            default.ok().expect("No files to open for config")
        };
    let mut reader = BufReader::new(&config);
    let mut buffer_string = String::new();
    reader.read_to_string(&mut buffer_string)
        .ok().expect("Failed to read in config");
    buffer_string
}
///Read Config Prompt
///Used to read the options of the config file and parse
///the defined options to create a customized prompt
pub fn read_config_prompt(input: &Prompt) -> String {
    let buffer_string = read_in_config();

    let value: toml::Value = buffer_string.parse()
        .expect("Should have a config file");
    let left = value.lookup("prompt.left")
        .expect("Expected value left in rusty.toml").as_str()
        .expect("Failed to convert to str").split("%");
    let mut prompt = "".to_owned();
    for i in left {
        if i.len() > 0 {
            match i.chars().nth(0).expect("Failed to parse string") {
                'U' => prompt.push_str(&var("USER").ok()
                                       .expect("No user env variable")),
                'H' => prompt.push_str(&String::from_utf8(Command::new("uname")
                                        .arg("-n").output()
                                        .ok()
                                        .expect("No uname command").stdout)
                                       .expect("Failed to convert to string")
                                       .trim()),
                'L' => prompt.push_str(&input.get_cwd()),
                'R' => {
                    let uid = String::from_utf8(Command::new("uname").arg("-n")
                                                .output().ok()
                                                .expect("No uname command")
                                                .stdout)
                        .ok().expect("Failed to convert string");
                    if uid == "0" {
                        prompt.push('#');
                    } else {
                        prompt.push('%');
                    }

                }
                 _ => prompt.push(i.chars().nth(0).expect("Failed to parse string")),
            }
        }
        //Add non Prompt special chars to prompt
        if i.len() > 1 {
            for j in 1 .. i.len() {
                prompt.push(i.chars().nth(j).expect("Failed to parse string"));
            }
        }
    }
    prompt.push(' ');
    prompt
}

///Check Alias
///Checks if there is an alias available before passing
///on commands for execution
pub fn check_alias(input: String) -> Option<String> {
    //Checks if alias is in config file and returns the altered
    //version as an Option of the input. If succesfully found
    //it can be unwraped for execution
    let input = input.split_whitespace().collect::<Vec<&str>>();
    //Makes sure there is something to execute
    if input.is_empty() {
        return None;
    }

    //Sets the alias to check for
    let alias_key = input.get(0).expect("Unwrapped an empty vector");

    //Check the config file for the key
    let config = read_in_config();
    let mut parsed = toml::Parser::new(&config)
        .parse().expect("Failed to pars config");
    let alias_table = parsed.remove("alias")
        .expect("Add an [alias] field to your config");
    let alias = alias_table.lookup(alias_key);

    //Checks if alias is in config file
    if !alias.is_some() {
        return None;
    }
    let output: String = toml::decode(alias
                        .expect("Already checked if alias is a none value")
                        .to_owned()).expect("Failed to decode value");
    Some(output)
}

///Set Env Var
///Sets system environment variables based on the configuration file
pub fn set_env_var() {
    let config = read_in_config();
    let mut parsed = toml::Parser::new(&config).parse().expect("Config parse unsuccessful");
    let env_table = parsed.remove("env_var")
        .expect("Add an [env_var] field to your config");

    //Grab all the keys, loop through, decode the value, and set the env variables
    let keys: Vec<_> = env_table.as_table().expect("Failed to convert to table").keys().cloned().collect();
    for key in keys {
        let value_unparsed: String = toml::decode(env_table.lookup(&key)
                                                  .expect("Failed lookup")
                                                  .to_owned())
            .expect("Failed to decode value");
        set_var(key,env_parse(value_unparsed));
    }
}

///Env Parse
///Given a value this parses it for expected behavior
/// #Example
/// PATH = PATH:~/.bin would tack on whatever is currently
/// in the PATH variable to ~/.bin
///
/// PATH = ~/.bin would just set PATH to ~/.bin
fn env_parse(input: String) -> String {
    //Take input string and add env variables to itself
    //e.g. PATH:~/.bin concats ~/.bin to PATH and returns it as
    //the new path variable
    let split_input: Vec<&str> = input.trim().split(':').collect();
    let mut output_vec: Vec<String> = Vec::new();

    //If it's a env variable gets the current value otherwise
    //pushes actual string to vector
    for i in split_input {
        let env_var = var(i.to_owned());
        if env_var.is_err() {
            output_vec.push(i.to_owned());
        } else {
            let env_var = env_var.expect("Env variabl does not exist");
            output_vec.push(env_var);
        }
    }

    //If it's just a single value it passes it back to set_var
    //in set_env_var
    if output_vec.len() == 1 {
        return output_vec.pop().expect("Called unwrap on an empty vec");
    }

    //Otherwise create a concatenated string of the values and returns that
    let mut output: String = String::new();
    for i in 0..output_vec.len() {
        if i > 0 {
            output.push_str(&format!(":{}",output_vec.get(i)
                                     .expect("Failed to format string")));
            continue;
        }
        output.push_str(&output_vec.get(i)
                        .expect("Called get on non existent value"));
    }
    output
}

/*//Stub for config file tests
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn readconfig() {
    }

}
*/
