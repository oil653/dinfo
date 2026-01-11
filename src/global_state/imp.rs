use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::clock::Clock;
use crate::weather_state::WeatherState;

#[derive(Default, Properties)]
#[properties[wrapper_type = super::GlobalState]]
pub struct GlobalState {
    #[property(get, set)]
    clock: RefCell<Option<Clock>>,
    #[property(get, set)]
    weather: RefCell<Option<WeatherState>>
}

#[glib::object_subclass]
impl ObjectSubclass for GlobalState {
    const NAME: &'static str = "GlobalState";
    type Type = super::GlobalState;
}

#[glib::derived_properties]
impl ObjectImpl for GlobalState {}