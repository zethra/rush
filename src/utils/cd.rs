#![allow(unused_must_use)] //supresses warning from use of env::var("Home")
use std::path::{Path,PathBuf};
use std::env;

//as_str() is considered unstable right now
//Need to work on what happens when mispelled directory is input
pub fn change_directory(input: Vec<&str>){
    if input.is_empty(){
        env::set_current_dir(Path::new(env::var("HOME").unwrap().as_str()));
    } else{
        let mut buffer = PathBuf::new();
        for i in input {
            buffer.push(Path::new(i));
        }
    
        let dir = buffer.as_path();
        if dir.is_relative(){
            let mut temp = PathBuf::new();
            temp.push(dir.parent().unwrap());
            temp.push(dir);
            env::set_current_dir(temp.as_path()).unwrap();
        } else {
            env::set_current_dir(dir).unwrap();
        }
    }
}



#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::env;
    use super::*;
    
    #[test]
    #[should_panic]
    fn test_change_directory_ok(){
        let vec = vec!["/"];
        let dir = Path::new("/tmp").to_str();
        change_directory(vec);
        let new_dir = env::current_dir().unwrap();
        let new_dir = new_dir.to_str();
        assert_eq!(dir, new_dir); 
    }

    #[test]
    fn test_change_directory_fail(){
        let vec = vec!["/"];
        let dir = Path::new("/").to_str();
        change_directory(vec);
        let new_dir = env::current_dir().unwrap();
        let new_dir = new_dir.to_str();
        assert_eq!(dir, new_dir); 
    }

}

