use std::path::Path;
use std::env;

pub fn change_directory(directory: &Path){
    let mut dir = directory;
    if dir.is_relative(){
        println!("dir is relative");
    } else {
        println!("dir is absolute");
    }

    env::set_current_dir(dir).unwrap();
}



#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::env;
    use super::*;
    
    #[test]
    #[should_panic]
    fn test_change_directory_ok(){
        let mut dir = Path::new("/tmp").to_str();
        change_directory(Path::new("/"));
        let new_dir = env::current_dir().unwrap();
        let new_dir = new_dir.to_str();
        assert_eq!(dir, new_dir); 
    }

    #[test]
    fn test_change_directory_fail(){
        let mut dir = Path::new("/").to_str();
        change_directory(Path::new("/"));
        let new_dir = env::current_dir().unwrap();
        let new_dir = new_dir.to_str();
        assert_eq!(dir, new_dir); 
    }

}

