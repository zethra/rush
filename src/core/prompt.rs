extern crate toml;

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
            cwd: "meh".to_string(),
        }
    }

    pub fn get_user_p(&self) -> String {
        self.user_p.to_owned()
    }

    //pub fn update_cwd(&self) -> String {
    //
    //}

}
