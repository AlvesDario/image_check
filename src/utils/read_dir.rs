use std::fs;

pub fn read_dir_recursive(path: &str) -> Vec<String> {
    let mut res = Vec::new();
    let dir_files = fs::read_dir(path);
    if dir_files.is_err() {
        println!("invalid path");
        return res;
    }

    for dir_entry in dir_files.unwrap() {
        match dir_entry {
            Ok(entry) => {
                let is_directory = entry.path().is_dir();
                let child_path = entry.path().as_os_str().to_str().unwrap().to_owned();

                match is_directory {
                    true => {
                        for dir in read_dir_recursive(&child_path) {
                            res.push(dir);
                        };

                    },
                    false => {
                        res.push(child_path.clone());
                    }
                }
            },
            Err(_) => (),
        }
    }
    return res;
}

pub fn to_json(dir_files: Vec<String>) {
    println!("[\n\t\"{}\"\n]", dir_files.join("\",\n\t\""));
}