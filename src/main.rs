use gtk::{ Align, Application, Box as Gbox, Button, CssProvider, Label, Overlay, gdk::Display, glib::{self, BindingFlags}, prelude::* };
use gtk_ls::{self, LayerShell, Layer};
use gtk::gdk;


mod weather;
mod units;
use crate::units::Units;
use units::{Speed, Precipitation, Temperature};


use std::sync::Arc;
use async_channel;
use tokio::runtime::Runtime;
use std::sync::OnceLock;

use chrono;
use std::time::Duration;


// Custom GObjects
mod global_state;
use global_state::GlobalState;

mod clock;
use clock::Clock;

mod weather_state;

const APP_ID: &str = "dinfo.oil653";

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}

fn build_ui(app: &Application, units: Arc<Units>) {
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
            }
        }
    ));

    let monitors = gdk::Display::default().expect("Failed to get all monitors").monitors();
    for monitor in monitors.iter().flatten() {
        let current_snd = current_snd.clone();
        let units = units.clone();


        //  =========> CLOCK <=========
        let clock1 = {
            Label::builder()
            .label("12:41:69")
            .name("clock1")
            .css_classes(["text", "clock"])
            .vexpand(false)
            .halign(Align::Center)
            .valign(Align::Center)
            .build()
        };

        clock_state
        .bind_property("clock1_label", &clock1, "label")
        .flags(BindingFlags::DEFAULT)
        .build();

        clock_state.connect_closure("clock1-should-fade", false, glib::closure_local!(
            #[weak]
            clock1,
            move |_obj: &Clock, new_state: bool| -> () {
                if new_state {
                    clock1.add_css_class("fade-out");
                } else {
                    clock1.remove_css_class("fade-out");
                }
            }
        ));

        let clock2 = { 
            Label::builder()
            .label("12:41:69")
            .name("clock2")
            .css_classes(["text", "clock"])
            .vexpand(false)
            .halign(Align::Center)
            .valign(Align::Center)
            .build()
        };

        clock_state
        .bind_property("clock2_label", &clock2, "label")
        .flags(BindingFlags::DEFAULT)
        .build();

        clock_state.connect_closure("clock2-should-fade", false, glib::closure_local!(
            #[weak]
            clock2,
            move |_obj: &Clock, new_state: bool| -> () {
                if new_state {
                    clock2.add_css_class("fade-out");
                } else {
                    clock2.remove_css_class("fade-out");
                }
            }
        ));

        let clock = {
            Overlay::builder()
            .halign(Align::Center)
            .valign(Align::Center)
            .child(&clock1)
            .build()
        };

        clock.add_overlay(&clock2);
        
        let date = {
            Label::builder()
            .label("2026/01/01")
            .name("date")
            .css_classes(["text"])
            .vexpand(false)
            .build()
        };

        let time_date_box = {
            Gbox::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(0)
            .build()
        };
        time_date_box.append(&clock);
        time_date_box.append(&date);

        //  =========> WEATHER <=========
        let wh = weather::CurrentWeather::new_example_with_code_fahrenheit(53);
        
        // CURRENT WEATHER
        // LEFT
        let current_weather_emoji = { 
            Label::builder()
            .label(wh.weather_code.to_emoji(wh.is_day))
            .css_classes(["emoji", "current_weather_title"])
            .justify(gtk::Justification::Left)
            .valign(Align::Center)
            .halign(Align::Start)
            .tooltip_text(format!("Cloud cover is {}%", wh.cloud_cover))
            .build()
        };

        let current_weather_temp = {
            Label::builder()
            .label(wh.temperature.to_string())
            .name("current_weather_temp")
            .css_classes(["text", "current_weather_title"])
            .build()
        };

        let current_weather_feels_like = {
            Label::builder()
            .label(format!("Feels like {}", wh.apparent_temp.to_string()))
            .css_classes(["text", "current_weather_text"])
            .tooltip_text("\"Feels like\" is calculated from the temperature, wind chill factor, relative humidity and solar radiation.")
            .build()
        };

        let current_weather_middle = {
            Gbox::builder()
            .orientation(gtk::Orientation::Vertical)
            .halign(Align::Center)
            .valign(Align::Center)
            .build()
        };
        current_weather_middle.append(&current_weather_temp);
        current_weather_middle.append(&current_weather_feels_like);

        // RIGHT
        let current_weather_humidity = {
            Label::builder()
            .label(format!("💧 {}%", wh.humidity))
            .css_classes(["text", "current_weather_text"])
            .tooltip_text("The relative humidity measured in the area")
            .build()
        };

        let current_weather_prec = {
            Label::builder()
            .label(format!("🌧️ {}", wh.precipitation.combined_to_string()))
            .css_classes(["text", "current_weather_text"])
            .tooltip_text(format!("Rain: {}\nShowers: {}\nSnowfall: {}", wh.precipitation.rain_to_string(), wh.precipitation.showers_to_string(), wh.precipitation.snowfall_to_string()))
            .build()
        };

        let current_weather_wind = {
            Label::builder()
            .label(format!("💨 {} {}", wh.wind.direction_stringify(), wh.wind.speed_stringify()))
            .css_classes(["text", "current_weather_text"])
            .build()
        };

        let current_weather_right = {
            Gbox::builder()
            .orientation(gtk::Orientation::Vertical)
            .halign(Align::End)
            .valign(Align::Center)
            .hexpand(true)
            .margin_end(10)
            .margin_top(5)
            .spacing(5)
            .build()
        };
        current_weather_right.append(&current_weather_humidity);
        current_weather_right.append(&current_weather_prec);
        current_weather_right.append(&current_weather_wind);


        let current_weather_data = {
            Gbox::builder()
            .orientation(gtk::Orientation::Horizontal)
            .halign(Align::Center)
            .hexpand(true)
            .build()
        };
        current_weather_data.append(&current_weather_emoji);
        current_weather_data.append(&current_weather_middle);
        current_weather_data.append(&current_weather_right);


        let current_weather_string = { 
            Label::builder()
            .label(wh.weather_code.to_string())
            .name("current_weather_string")
            .css_classes(["text"])
            .justify(gtk::Justification::Left)
            .halign(Align::Center)
            .build() 
        };

        let current_weather = {
            Gbox::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(0)
            .css_classes(["island"])
            .halign(Align::Center)
            .hexpand(true)
            .build()
        };
        current_weather.append(&current_weather_data);
        current_weather.append(&current_weather_string);

        // WEATHER UPDATE LOGIC

        // Parses current weather and sends it on channel, so it can be put in state
        let parse_weather = {
            let state = current_weather_state.clone();

            move || {
                if state.is_parsing() {
                    return
                } else {
                    state.set_is_parsing(true);
                }

                let current_snd = current_snd.clone();
                let units = units.clone();

                runtime().spawn(
                    async move {
                        let data = match weather::get_current_weather(&units).await {
                            Ok(d) => Some(d),
                            Err(e) => {
                                println!("{}", e.to_string());
                                None
                            }
                        };
                        println!("Got current weather");
                        current_snd.send(data).await.expect("Tried to send current  weather data on async channel");
                    }
                );
            }
        };

        current_weather_state.connect_is_parsing_notify(glib::clone!(
            #[weak] current_weather_emoji,
            #[weak] current_weather_temp,
            #[weak] current_weather_feels_like,
            #[weak] current_weather_humidity,
            #[weak] current_weather_prec,
            #[weak] current_weather_wind,
            #[weak] current_weather_string,
            move |state: &weather_state::WeatherState| {
                if !state.is_parsing() {
                    if let Some(wh) = state.get_current() {
                        current_weather_emoji.set_label(&wh.weather_code.to_emoji(wh.is_day));
                        current_weather_emoji.set_tooltip_text(Some(&format!("Cloud cover is {}%", wh.cloud_cover)));

                        current_weather_temp.set_label(&wh.temperature.to_string());

                        current_weather_feels_like.set_label(&format!("Feels like {}", wh.apparent_temp.to_string()));

                        current_weather_humidity.set_label(&format!("💧 {}%", wh.humidity));
                        current_weather_humidity.set_tooltip_text(Some("The relative humidity measured in the area"));

                        current_weather_prec.set_label(&format!("🌧️ {}", wh.precipitation.combined_to_string()));
                        current_weather_prec.set_tooltip_text(Some(&format!(
                            "Rain: {}\nShowers: {}\nSnowfall: {}",
                            wh.precipitation.rain_to_string(),
                            wh.precipitation.showers_to_string(),
                            wh.precipitation.snowfall_to_string()
                        )));

                        current_weather_wind.set_label(&format!("💨 {} {}", wh.wind.direction_stringify(), wh.wind.speed_stringify()));

                        current_weather_string.set_label(&wh.weather_code.to_string());
                    } else {
                        println!("Empty weather data received, leaving ui state untouched");
                    }
                } else {
                    println!("Parsing current weather data!")
                }
            }
        ));

        // Get the weather data every 15 minutes. 
        glib::spawn_future_local({
            let parse = parse_weather.clone();
            async move {
                loop{
                    parse();
                    glib::timeout_future(std::time::Duration::from_mins(15)).await;
                }
            }
        });

        // Button to update the current weather                                 DEBUG
        let update_button = Button::with_label("Update weather");
        update_button.connect_clicked(move |_| parse_weather());
        // =======================

        // ROOT of WEATHER
        let weather_box = Gbox::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_start(5)
            .margin_end(5)
            .margin_top(10)
            .build();

        weather_box.append(&current_weather);
        weather_box.append(&update_button); //                           DEBUG

        //  =========> ROOT <=========
        // Main box, root of the app
        let main_box = Gbox::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        main_box.append(&time_date_box);
        main_box.append(&weather_box);

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
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() -> glib::ExitCode {
    // This is some magic variable that needs to be set, to cut the ram usage in half
    unsafe {
        std::env::set_var("GSK_RENDERER", "cairo");
    }
    let units = Arc::new(Units::new(Speed::Kmh, Temperature::Celsius, Precipitation::Mm));

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app: &Application| {
        build_ui(app, Arc::clone(&units));
    });

    app.run()
}