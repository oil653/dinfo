use gtk::{ Application, Box as Gbox, CssProvider, gdk::Display, glib, prelude::* };
use gtk_ls::{self, LayerShell, Layer};
use gtk::gdk;


mod weather;
mod units;
use crate::{ui_elements::build_current_weather, units::Units};
use units::{Speed, Precipitation, Temperature};


use async_channel;
use std::sync::OnceLock;

use chrono;
use std::time::Duration;

mod ui_elements;
use ui_elements::{build_clock};

// Custom GObjects
mod global_state;
use global_state::GlobalState;

mod clock;

mod weather_state;


const APP_ID: &str = "dinfo.oil653";
static DATE_FORMAT: OnceLock<String> = OnceLock::new();
static UNITS: OnceLock<Units> = OnceLock::new();

fn build_ui(app: &Application) {
    let state = GlobalState::new();
    let clock_state = state.clock().clone().expect("Clock state returned None. (This shouldnt happen)");
    let current_weather_state = state.weather().clone().expect("Clock state returned None. (This shouldnt happen)");

    // Parse current weather data
    let (current_snd, current_rcv) = async_channel::bounded(1);

    // Sets current weather state
    glib::spawn_future_local(glib::clone!(
        #[strong]
        current_weather_state,
        async move {
            while let Ok(weather) = current_rcv.recv().await {
                current_weather_state.set_current(weather);
                current_weather_state.set_is_parsing(false);
                // println!("stopped parsing. is_parsing: {}", current_weather_state.is_parsing());
            }
        }
    ));

    let monitors = gdk::Display::default().expect("Failed to get all monitors").monitors();
    for monitor in monitors.iter().flatten() {
        // Channel needed to send to weather data
        let current_snd = current_snd.clone();

        let main_box = Gbox::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let clock = build_clock(clock_state.clone());


        let ( current_weather, update_internal) = build_current_weather(&current_weather_state, current_snd);

        current_weather_state.connect_is_parsing_notify(move |_| {
            update_internal();
        });

        main_box.append(&clock);
        main_box.append(&current_weather);

        //  =========> WINDOW <=========
        let window = gtk::ApplicationWindow::builder()
        .name("main_window")
        .default_height(600)
        .default_width(500)
        .application(app)
        .show_menubar(false)
        .title("dinfo - a desktop info thing")
        .child(&main_box)
        .build();

        // Set up layer-shell
        window.init_layer_shell();
        window.set_layer(Layer::Bottom);
        window.set_exclusive_zone(0);
        window.set_monitor(Some(&monitor));

        window.present();

        println!("{}", window.color());
    }

    // Animate, and drive the clock state changes
    glib::spawn_future_local(async move {
        let state = clock_state;
        // println!("{:?}", state);
        loop {
            let now = chrono::Local::now();
            state.set_clock2_label(now.format("%H:%M:%S").to_string());
            state.clock2_should_fade(false);
            glib::timeout_future(Duration::from_millis(50)).await;
            state.clock1_should_fade(true);

            glib::timeout_future(Duration::from_millis(950)).await;

            let now = chrono::Local::now();
            state.set_clock1_label(now.format("%H:%M:%S").to_string());
            state.clock1_should_fade(false);
            glib::timeout_future(Duration::from_millis(50)).await;
            state.clock2_should_fade(true);

            glib::timeout_future(Duration::from_millis(950)).await;
        }
    });
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));
    
    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
}

fn main() -> glib::ExitCode {
    // This is some magic variable that needs to be set, to cut the ram usage in half
    unsafe {
        std::env::set_var("GSK_RENDERER", "cairo");
    }

    UNITS.set(Units::new(Speed::Kmh, Temperature::Celsius, Precipitation::Mm)).expect("Failed to set UNITS static");
    DATE_FORMAT.set("%d/%m/%Y".to_string()).expect("Failed to set DATE_FORMAT static");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}