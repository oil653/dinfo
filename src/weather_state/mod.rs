use glib::Object;
use gtk::glib::{self, subclass::types::ObjectSubclassIsExt};

mod imp;

use crate::weather::CurrentWeather;

glib::wrapper!{
    pub struct WeatherState(ObjectSubclass<imp::WeatherState>);
}

impl WeatherState {
    pub fn new() -> Self {
        Object::builder()
            .property("is_parsing", false)
            .build()
    }

    pub fn set_current(&self, new_value: Option<CurrentWeather>) {
        self.imp().set_current(new_value)
    }

    pub fn get_current(&self) -> Option<CurrentWeather> {
        self.imp().get_current()
    }
}