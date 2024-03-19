use std::fs;


mod counter_app;


fn read_dir_recursive(path: &str) -> Option<bool> {
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

fn main() {
    read_dir_recursive("./src");
    // Create a new application with the builder pattern
    let app = Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(counter_app::counter_app);
    // Run the application
    app.run();
}
