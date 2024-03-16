use gtk::glib;
use gtk::{prelude::*, Application, ApplicationWindow};
use std::{cell::Cell, rc::Rc};

pub fn counter_app(app: &Application) {
    let button_increase = gtk::Button::builder()
        .label("increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = gtk::Button::builder()
        .label("decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number = Rc::new(Cell::new(0));

    let counter_label = gtk::Label::builder()
        .label(&number.get().to_string())
        .build();

    button_increase.connect_clicked(
        glib::clone!(@strong number, @weak counter_label => move |_| {
            number.set(number.get() + 1);
            counter_label.set_label(&number.get().to_string());
        }),
    );
    button_decrease.connect_clicked(glib::clone!(@weak counter_label => move |_| {
        number.set(number.get() - 1);
        counter_label.set_label(&number.get().to_string());
    }));

    let button_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    button_box.append(&counter_label);
    button_box.append(&button_decrease);
    button_box.append(&button_increase);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("count")
        .child(&button_box)
        .build();
    window.present();
}
