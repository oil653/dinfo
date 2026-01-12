use gtk::{ Align, Box as Gbox, Button, GestureClick, Label, Overlay, glib::{self, BindingFlags}, prelude::* };

use crate::{ clock::Clock, weather::CurrentWeather };
use crate::weather_state::WeatherState;

use chrono::{ self, Local };
use std::time::Duration;

use tokio::runtime::Runtime;
use async_channel;
use std::sync::OnceLock;

use crate::weather;

fn get_seconds_to_midnight() -> i64 {
    let now = Local::now();

    // Get todays midnight (most likely passed this already)
    let mut midnigth = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    // Add one day so we get the first second of the new day
    midnigth += Duration::from_hours(24);

    (midnigth - now.naive_local()).num_seconds()
}

fn get_today_date() -> String {
    Local::now().format(crate::DATE_FORMAT.get().unwrap()).to_string()
}

pub fn build_clock(clock_state: Clock) -> Gbox {
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
            .label(get_today_date())
            .name("date")
            .css_classes(["text", "emoji"])
            .vexpand(false)
            .build()
        };

        glib::spawn_future_local(glib::clone!(
            #[weak]
            date,
            async move {
                // println!("{} seconds till midnight", get_seconds_to_midnight());
                glib::timeout_future_seconds(get_seconds_to_midnight() as u32).await;

                date.set_label(get_today_date().as_str());

                loop {
                    glib::timeout_future_seconds(get_seconds_to_midnight() as u32).await;
                    date.set_label(get_today_date().as_str());
                }
            }
        ));

        let time_date_box = {
            Gbox::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(0)
            .build()
        };
        time_date_box.append(&clock);
        time_date_box.append(&date);

        time_date_box
}


fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}

pub fn build_current_weather(
    current_weather_state: &WeatherState, 
    weather_result_sender: async_channel::Sender<Option<CurrentWeather>>, 
) -> (Gbox, Box<dyn Fn()>) {
    // Base weather data to initialise the ui with
    let wh = weather::CurrentWeather::new_example_with_code(0);

    let current_weather_state = current_weather_state.clone();

    // LEFT
    let weather_emoji = { 
        Label::builder()
        .label(wh.weather_code.to_emoji(wh.is_day))
        .css_classes(["emoji", "current_weather_title"])
        .justify(gtk::Justification::Left)
        .valign(Align::Center)
        .halign(Align::Start)
        .tooltip_text(format!("Cloud cover is {}%", wh.cloud_cover))
        .build()
    };

    let temp = {
        Label::builder()
        .label(wh.temperature.to_string())
        .name("current_weather_temp")
        .css_classes(["text", "current_weather_title"])
        .build()
    };

    let feels_like = {
        Label::builder()
        .label(format!("Feels like {}", wh.apparent_temp.to_string()))
        .css_classes(["text", "current_weather_text"])
        .tooltip_text("\"Feels like\" is calculated from the temperature, wind chill factor, relative humidity and solar radiation.")
        .build()
    };

    let temp_box = {
        Gbox::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(Align::Center)
        .valign(Align::Center)
        .build()
    };
    temp_box.append(&temp);
    temp_box.append(&feels_like);

    // RIGHT
    let humidity = {
        Label::builder()
        .label(format!("💧 {}%", wh.humidity))
        .css_classes(["text", "current_weather_text"])
        .tooltip_text("The relative humidity measured in the area")
        .build()
    };

    let prec = {
        Label::builder()
        .label(format!("🌧️ {}", wh.precipitation.combined_to_string()))
        .css_classes(["text", "current_weather_text"])
        .tooltip_text(format!("Rain: {}\nShowers: {}\nSnowfall: {}", wh.precipitation.rain_to_string(), wh.precipitation.showers_to_string(), wh.precipitation.snowfall_to_string()))
        .build()
    };

    let wind = {
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
    current_weather_right.append(&humidity);
    current_weather_right.append(&prec);
    current_weather_right.append(&wind);


    let current_weather_data = {
        Gbox::builder()
        .orientation(gtk::Orientation::Horizontal)
        .halign(Align::Center)
        .hexpand(true)
        .build()
    };
    current_weather_data.append(&weather_emoji);
    current_weather_data.append(&temp_box);
    current_weather_data.append(&current_weather_right);


    let status_string = { 
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
        .margin_start(5)
        .margin_end(5)
        .margin_top(10)
        .build()
    };
    current_weather.append(&current_weather_data);
    current_weather.append(&status_string);

    let retry = {
        Button::builder()
            .label("🔁")
            .css_name("weather_repeat")
            .tooltip_text("Retry parsing weather data")
            .margin_bottom(5)
            .margin_top(5)
            .margin_start(5)
            .margin_end(5)
            .build()
    };

    // WEATHER UPDATE LOGIC

    let update_ui = Box::new(glib::clone!(
        #[strong] weather_emoji,
        #[strong] temp,
        #[strong] feels_like,
        #[strong] humidity,
        #[strong] prec,
        #[strong] wind,
        #[strong] status_string,
        #[strong] retry,
        #[strong] current_weather,
        #[strong] current_weather_state,
        move || {
            // println!("IS_PARSING connection connected, yippi. {:?}", current_weather_state.get_current());
            if let Some(wh) = current_weather_state.get_current() {
                // Remove the network error retry button, and add back the normal interface if the parse succeeds
                if let Some(parent) = retry.parent(){
                    if let Some(gbox) = parent.downcast_ref::<Gbox>(){
                        if *gbox == current_weather {
                            current_weather.remove(&retry);

                            current_weather.append(&current_weather_data);
                            current_weather.append(&status_string);
                        }
                    }
                }

                let combined_time_format = format!("{} %H:%M:%S", crate::DATE_FORMAT.get().unwrap());
                let current_time_date = Local::now().format(combined_time_format.as_str());
                let last_updated = format!("Last updated: {}", current_time_date);
                current_weather.set_tooltip_text(Some(last_updated.as_str()));

                weather_emoji.set_label(&wh.weather_code.to_emoji(wh.is_day));
                weather_emoji.set_tooltip_text(Some(&format!("Cloud cover is {}%", wh.cloud_cover)));

                temp.set_label(&wh.temperature.to_string());

                feels_like.set_label(&format!("Feels like {}", wh.apparent_temp.to_string()));

                humidity.set_label(&format!("💧 {}%", wh.humidity));
                humidity.set_tooltip_text(Some("The relative humidity measured in the area"));

                prec.set_label(&format!("🌧️ {}", wh.precipitation.combined_to_string()));
                prec.set_tooltip_text(Some(&format!(
                    "Rain: {}\nShowers: {}\nSnowfall: {}",
                    wh.precipitation.rain_to_string(),
                    wh.precipitation.showers_to_string(),
                    wh.precipitation.snowfall_to_string()
                )));

                wind.set_label(&format!("💨 {} {}", wh.wind.direction_stringify(), wh.wind.speed_stringify()));

                status_string.set_label(&wh.weather_code.to_string());
            } else {
                println!("Empty weather data recieved :(");

                // Check if current_weather_data and status string is parents
                // of the box before trying to remove it to avoid errors
                if let Some(parent) = current_weather_data.parent() {
                    if let Some(gbox) = parent.downcast_ref::<Gbox>() {
                        if *gbox == current_weather {
                            current_weather.remove(&current_weather_data);
                            current_weather.remove(&status_string);

                            current_weather.append(&retry);
                        }
                    }
                }

                weather_emoji.set_label("");
                weather_emoji.set_tooltip_text(None);

                temp.set_label("");

                feels_like.set_label("");

                humidity.set_label("");
                humidity.set_tooltip_text(None);

                prec.set_label("");
                prec.set_tooltip_text(None);

                wind.set_label("");

                status_string.set_label("");
            }
        }
    ));

    // Parses current weather and sends it on channel, so it can be put in state
    let parse_weather = {
        let state = current_weather_state.clone();

        move || {
            if state.is_parsing() {
                return
            } else {
                state.set_is_parsing(true);
            }

            let current_snd = weather_result_sender.clone();

            runtime().spawn(
                async move {
                    let data = match weather::get_current_weather(crate::UNITS.get().unwrap()).await {
                        Ok(d) => {
                            println!("Got current weather");
                            Some(d)
                        },
                        Err(e) => {
                            println!("Failed to get weather data: {}", e.to_string());
                            None
                        }
                    };
                    // println!("{:?}", data);
                    current_snd.send(data).await.expect("Tried to send current  weather data on async channel");
                }
            );
        }
    };

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

    // Handle clicking in the widget
    let click = GestureClick::new();
    click.connect_pressed(move |_, _, _, _| parse_weather());
    current_weather.add_controller(click);

    (current_weather, update_ui)
}