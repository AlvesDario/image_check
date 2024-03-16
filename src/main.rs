use gtk::{prelude::*, Application};

mod counter_app;

fn main() {
    // Create a new application with the builder pattern
    let app = Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(counter_app::counter_app);
    // Run the application
    app.run();
}
