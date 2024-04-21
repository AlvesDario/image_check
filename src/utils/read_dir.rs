use std::fs;

pub fn read_dir_recursive(path: &str) -> Vec<String> {
    let dir_files = fs::read_dir(path);
    if dir_files.is_err() {
        println!("invalid path");
        return Vec::new();
    }

    for dir_entry in dir_files.unwrap() {
        match dir_entry {
            Ok(entry) => {
                let is_directory = entry.path().is_dir();
                let child_path = entry.path().as_os_str().to_str().unwrap().to_owned();
        
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
    return Vec::new();
}