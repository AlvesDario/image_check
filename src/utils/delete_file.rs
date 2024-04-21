use std::{fs, io::Read};

pub fn delete_file(path: &str) {
    let found_item = fs::metadata(path).expect("Couldn't read path");
    if found_item.is_dir() {
        println!("that's a directory");
        println!("deleting directory: {}", path);
        fs::remove_dir_all(path).expect("couldn't delete folder");
        return;
    }
    println!("that's a file");
    println!("deleting file: {}", path);
    fs::remove_file(path).expect("couldn't delete file");
}