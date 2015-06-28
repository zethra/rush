extern crate toml;

//For now it will access the toml file in the config directory
//Later give it the means to access it from ~/.rusty.toml
//Activate this only when performing su or cd that way it reduces
//need to keep getting the prompt
pub fn get_prompt() -> String {
    
    "Placeholder".to_owned()        
}
