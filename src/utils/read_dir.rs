use std::fs;

pub fn read_dir_recursive(path: &str) -> Option<bool> {
    let dir_files = fs::read_dir(path).ok()?;
    
    for dir_entry in dir_files {
        match dir_entry {
            Ok(entry) => {
                let is_directory = entry.path().is_dir();
                let child_path = entry.path().as_os_str().to_str()?.to_owned();
        
                match is_directory {
                    true => {
                        read_dir_recursive(&child_path);
                    },
                    false => {
                        println!("{}", child_path);
                    }
                }
            },
            Err(_) => (),
        }
    }
    return Some(true);
}