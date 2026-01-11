use std::cell::RefCell;

use std::sync::OnceLock;
use glib::subclass::Signal;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default, Properties)]
#[properties[wrapper_type = super::Clock]]
pub struct Clock {
    #[property(get, set)]
    clock1_label: RefCell<String>,
    #[property(get, set)]
    clock2_label: RefCell<String>
}

impl Clock {
    pub fn clock1_should_fade(&self, should_fade: bool) {
        self.obj().emit_by_name::<()>("clock1-should-fade", &[&should_fade]);
    }

    pub fn clock2_should_fade(&self, should_fade: bool) {
        self.obj().emit_by_name::<()>("clock2-should-fade", &[&should_fade]);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Clock {
    const NAME: &'static str = "ClockState";
    type Type = super::Clock;
}

#[glib::derived_properties]
impl ObjectImpl for Clock {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("clock1-should-fade")
                    .param_types([bool::static_type()])
                    .build(),
                Signal::builder("clock2-should-fade")
                    .param_types([bool::static_type()])
                    .build(),
            ]
        })
    }
}