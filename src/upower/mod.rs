use std::error::Error;

use upower_dbus::{ self, DeviceProxy };
use zbus::{ PropertyStream, Connection };

use gtk::glib;

use std::sync::{ Arc, Mutex };

mod helpers;

// GObject
mod gobject;
pub use gobject::UPower;

#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub percentage: f64,
    pub icon_name: String,
    pub percentage_changed: Arc<Mutex<PropertyStream<'static, f64>>>
}

impl Device {
    pub async fn new(connection: &Connection, device_path: String) -> Result<Self, Box<dyn Error>> {
        let device = DeviceProxy::new(&connection, device_path).await?;
        
        let name = device.model().await?;
        let percentage = device.percentage().await?;
        let icon_name = helpers::get_icon(&device, &connection).await;
        let percentage_changed: Arc<Mutex<PropertyStream<'_, f64>>> = Arc::new(Mutex::new(device.receive_percentage_changed().await));

        Ok(Device { name, percentage, icon_name, percentage_changed })
    }

    pub async fn poll_self<F>(&self, F: F) {
        // Yet to be implemented
    }
}