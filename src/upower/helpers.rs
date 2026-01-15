use std::error::Error;

use upower_dbus::{ self, DeviceProxy }; 
use zbus::Connection;

use std::collections::HashMap;

// use futures::stream::StreamExt;

// For more information about this, check out my prototype repo: 
// https://github.com/oil653/upower-example-rs.git

/// Check if a device is a bluez device or not
pub async fn is_bluez(dev: &DeviceProxy<'_>) -> bool {
    match dev.native_path().await {
        Ok(path) => path.starts_with("/org/bluez"),
        Err(e) => {
            eprint!("Failed to get native path for a device: {e}");
            false
        }
    }
}

/// Attempts to request the icon name from bluez
pub async fn get_bluez_icon(connection: &Connection, native_path: &str) -> Result<Option<String>, Box<dyn Error>> {
    let message = connection.call_method(
        Some("org.bluez"), 
        native_path, 
        Some("org.freedesktop.DBus.Properties"), 
        "GetAll",
        &("org.bluez.Device1")
    )
    .await?;

    // Deserialize the DBus data to a rust Hashmap of String: Values
    let reply = message.body::<HashMap<String, zbus::zvariant::Value>>()?;

    // Try to get the "Icon" field from the hashmap
    match reply.get("Icon") {
        Some(value) => Ok(Some(value.to_string())),
        None => {
            eprintln!("Got bluez path, but it did not contain Icon fields");
            Ok(None)
        }
    }
}

/// Get the device's icon name from either UPower or Bluez
/// If it fails it logs to stderr and returns "battery-missing-symbolic" as a fallback
pub async fn get_icon(device: &DeviceProxy<'_>, connection: &Connection) -> String {
    let icon_name = match device.icon_name().await {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to get icon_name for a upower device: {e}");
            return "battery-missing-symbolic".to_string()
        } 
    };

    if icon_name == "battery-missing-symbolic" && is_bluez(&device).await {
        let native_path= match device.native_path().await {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to get native path: {e}");
                return "battery-missing-symbolic".to_string();
            }
        };

        // Tries to get the path from Bluez
        match get_bluez_icon(&connection, &native_path).await {
            Ok(icon) => {
                match icon {
                    Some(icon) => icon,
                    None => {
                        eprintln!("GetAll on bluez device with path '{}' succeded, but Icon field were empty", native_path);
                        icon_name
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to get icon name from bluez: {e}");
                icon_name
            }
        }
    } else {
        icon_name
    }
}