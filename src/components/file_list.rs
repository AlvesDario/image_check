use gtk::prelude::BoxExt;

pub struct FileList {
    items: Vec<String>,
}

impl FileList {
    pub fn new(files: Vec<String>) -> Self {
        return Self {
            items: files,
        }
    }

    pub fn build_widget(&self) -> gtk::Box {
        let body = gtk::Box::new(gtk::Orientation::Vertical, i32::try_from(self.items.len()).expect("failed to convert file list to i32"));
        for item in &self.items {
            body.prepend(&gtk::Label::new(Some(&item)));
        }
        return body;
    }
}