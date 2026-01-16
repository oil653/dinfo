use glib::Object;
use gtk::glib::{self, object::ObjectExt, subclass::types::ObjectSubclassIsExt};

use upower_dbus::UPowerProxy;
use zbus::{Connection, zvariant::OwnedObjectPath};

use futures::{ select, stream::StreamExt, pin_mut };

mod imp;

use super::Device;

// The way this works: 
// Devices are automatically added and removed from a Vec (the Vec is fully remade)
// The individual devices are lazy, they dont update their state, but they contain the stream you can await
// on to get the percentage change

glib::wrapper!{
    pub struct UPower(ObjectSubclass<imp::UPower>);
}

impl UPower {
    pub async fn new() -> Self {
        let obj: UPower = Object::builder()
        .property("init_success", false)
        .build();

        let connection= match Connection::system().await {
            Ok(c) => {
                println!("Successfully connected to system DBus");
                c
            },
            Err(e) => {
                eprintln!("Failed to connect to system DBus bus: {e}");
                return obj
            }
        };

        let upower = match UPowerProxy::new(&connection).await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to create UPower proxy while creating new UPower GObject: {e}");
                return obj;
            }
        };

        let device_paths = match upower.enumerate_devices().await {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to run enumerate_devices() from upower proxy: {e}");
                return obj;
            }
        };

        let devices: Vec<Device> = Self::parse_devices(&connection, &device_paths).await;

        let imp = obj.imp();
        {
            let mut guard = imp.devices.lock().await;
            *guard = devices;
        }
        imp.connection.replace(Some(connection));

        obj.set_property("init_success", true);

        let uproxy = upower.clone();
        glib::spawn_future_local(glib::clone!(
            #[strong]
            obj,
            async move {
                let update_devices = async move || {
                    let connection = obj.imp().connection.borrow();
                    let connection = connection.as_ref().expect("Connection is None, expected to be Some");

                    let device_paths = match upower.enumerate_devices().await {
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!("Failed to run enumerate_devices() from upower proxy: {e}");
                            Vec::new()
                        }
                    };
                    
                    let devices: Vec<Device> = Self::parse_devices(&connection, &device_paths).await;

                    let imp = obj.imp();
                    {
                        let mut guard = imp.devices.lock().await;
                        *guard = devices;
                    }

                    obj.emit_by_name::<()>("devices-changed", &[]);
                };

                let added = match uproxy.receive_device_added().await{
                    Ok(d) => d.fuse(),
                    Err(e) => {
                        eprintln!("Failed to get device_added signal stream: {e}");
                        return;
                    }
                };
                let removed = match uproxy.receive_device_removed().await {
                    Ok(d) => d.fuse(),
                    Err(e) => {
                        eprintln!("Failed to get device_removed signal stream: {e}");
                        return;
                    }
                };

                pin_mut!(added, removed);

                let mut added_finished = false;
                let mut removed_finished = false;
                loop {
                    if added_finished && removed_finished {
                        eprintln!("Both added and removed stream on UPower finished. Device add/removes will not update the state automatically");
                        break;
                    }

                    if added_finished {
                        eprintln!("Added stream on UPower finished. Device adds will not update the state automatically")
                    }

                    if removed_finished {
                        eprintln!("Removed stream on UPower finished. Device removes will not update the state automatically")
                    }

                    select! {
                        item = added.next() => {
                            match item {
                                Some(_) => {
                                    // do stuff
                                    println!("Meow");
                                    update_devices().await;
                                }
                                None => {
                                    added_finished = true;
                                }
                            }
                        },
                        item = removed.next() => {
                            match item {
                                Some(_) => {
                                    // do stuff
                                    println!("Meow");
                                    update_devices().await;
                                }
                                None => {
                                    removed_finished = true;
                                }
                            }
                        }
                    }
                }
            })
        );

        obj
    }

    async fn parse_devices(connection: &Connection, device_paths: &Vec<OwnedObjectPath>) -> Vec<Device> {
        let mut devices: Vec<Device> = Vec::new();

        for device in device_paths {
            let device = match Device::new(&connection, device.to_string()).await {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Failed to create device struct from {device}: {e}");
                    continue;
                }
            };
            devices.push(device);
        }

        devices
    }

    pub async fn get_devices(&self) -> Vec<Device> {
        self.imp().get_devices().await
    }
}