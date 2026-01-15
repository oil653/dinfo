use std::error::Error;

use upower_dbus::{ self, DeviceProxy, UPowerProxy };
use zbus::{ PropertyStream, Connection };

use std::sync::{ Arc, Mutex };

mod helpers;

#[derive(Debug)]
pub struct Device<'a> {
    pub name: String,
    pub percentage: f64,
    pub icon_name: String,
    pub percentage_changed: Arc<Mutex<PropertyStream<'a, f64>>>
}

impl<'a> Device<'a> {
    pub async fn new(connection: &Connection, device_path: String) -> Result<Self, Box<dyn Error>> {
        let device = DeviceProxy::new(&connection, device_path).await?;
        
        let name = device.model().await?;
        let percentage = device.percentage().await?;
        let icon_name = helpers::get_icon(&device, &connection).await;
        let percentage_changed: Arc<Mutex<PropertyStream<'_, f64>>> = Arc::new(Mutex::new(device.receive_percentage_changed().await));

        Ok(Device { name, percentage, icon_name, percentage_changed })
    }
}