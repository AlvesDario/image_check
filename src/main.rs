use gtk::{prelude::*, Application, ApplicationWindow};
use crate::utils::{read_dir::read_dir_recursive, delete_file::delete_file};
use crate::components::file_list;

mod counter_app;
mod utils;
mod components;

// When the application is launched…
fn on_activate(application: &Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Should it be deleted?")
        .build();


    let app_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    
    let entry = gtk::Entry::new();
    app_box.append(&entry);
    {
        let yes_button = gtk::Button::builder()
            .label("yes!")
            .margin_start(20)
            .margin_end(20)
            .build();

        let entry = entry.clone();
        let box_clone = app_box.clone();
        yes_button.connect_clicked(move |_| {
            println!("clicou em sim {}", entry.text());
            let label_list = file_list::FileList::new(read_dir_recursive(&entry.text()));
            box_clone.append(&label_list.build_widget());
        });
        app_box.append(&yes_button);
    }
    {
        let no_button = gtk::Button::builder()
            .label("No!")
            .margin_start(20)
            .margin_end(20)
            .build();

        let entry = entry.clone();
        no_button.connect_clicked(move |_| {
            println!("clicou em sim {}", entry.text());
            read_dir_recursive(&entry.text());
        });
        app_box.append(&no_button);
    }

    let btn_label = gtk::Label::builder().label("clique se ele deu").build();

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Should it be deleted?")
        .child(&app_box)
        .build();
    window.present();
}


fn main() {
    // Create a new application with the builder pattern
    let app = Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(on_activate);
    // Run the application
    app.run();
}
