use glib::Object;
use gtk::glib;

mod imp;

use crate::clock::Clock;
use crate::weather_state::WeatherState;

glib::wrapper!{
    pub struct GlobalState(ObjectSubclass<imp::GlobalState>);
}

impl GlobalState {
    pub fn new() -> Self {
        let obj = Object::builder()
        .property("clock", Some(Clock::new()))
        .property("weather", Some(WeatherState::new()))
        .build();

        glib::spawn_future_local(async move {
        });

        obj
    }
}