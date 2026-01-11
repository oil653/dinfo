use std::cell::RefCell;
use std::cell::Cell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::weather::CurrentWeather;

#[derive(Default, Properties, Debug)]
#[properties[wrapper_type = super::WeatherState]]
pub struct WeatherState {
    /// True if the weather is currently being parsed, so it shouldnt be read
    #[property(get, set)]
    is_parsing: Cell<bool>,
    current: RefCell<Option<CurrentWeather>>
}

impl WeatherState {
    pub fn get_current(&self) -> Option<CurrentWeather> {
        self.current.borrow().clone()
    }

    pub fn set_current(&self, new_value: Option<CurrentWeather>) {
        self.current.replace(new_value);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for WeatherState {
    const NAME: &'static str = "WeatherState";
    type Type = super::WeatherState;
}

#[glib::derived_properties]
impl ObjectImpl for WeatherState {}