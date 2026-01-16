use std::cell::RefCell;
use std::sync::OnceLock;

use async_lock::Mutex;

use glib::Properties;
use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::super::Device;
use zbus::Connection;

#[derive(Default, Properties)]
#[properties[wrapper_type = super::UPower]]
pub struct UPower {
    #[property(get, set)]
    init_success: RefCell<bool>,
    pub devices: Mutex<Vec<Device>>,
    pub connection: RefCell<Option<Connection>>
}

impl UPower {
    pub async fn get_devices(&self) -> Vec<Device> {
        let guard = self.devices.lock().await;
        (*guard.clone()).to_vec()
    }
}

#[glib::object_subclass]
impl ObjectSubclass for UPower {
    const NAME: &'static str = "UPower";
    type Type = super::UPower;
}

#[glib::derived_properties]
impl ObjectImpl for UPower {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("devices-changed").build(),
            ]
        })
    }
}