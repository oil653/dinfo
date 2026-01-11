use glib::Object;
use gtk::glib::{self, subclass::types::ObjectSubclassIsExt};

mod imp;

glib::wrapper!{
    pub struct Clock(ObjectSubclass<imp::Clock>);
}

impl Clock {
    pub fn new() -> Self {
        Object::builder()
        .property("clock1_label", String::new())
        .property("clock2_label", String::new())
        .build()
    }

    pub fn clock1_should_fade(&self, new_state: bool) {
        self.imp().clock1_should_fade(new_state);
    }

    pub fn clock2_should_fade(&self, new_state: bool) {
        self.imp().clock2_should_fade(new_state);
    }
}