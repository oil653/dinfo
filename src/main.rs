use gtk::{
    Align, Application, Box as Gbox, Button, CssProvider, Label, gdk::Display, glib, prelude::*
};
use gtk_ls::{self, LayerShell, Layer};

use async_channel;

use crate::weather::CurrentWeather;

mod weather;
mod units;

const APP_ID: &str = "dinfo.oil653";

fn build_ui(app: &Application) {
//  =========> CLOCK <=========
    let clock_main_box = Gbox::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(0)
        .build();

    let clock = Label::builder()
        .label("12:41:69")
        .name("clock")
        .css_classes(["text"])
        .vexpand(false)
        .build();

    clock_main_box.append(&clock);



    let date = Label::builder()
        .label("2026/01/01")
        .name("date")
        .css_classes(["text"])
        .vexpand(false)
        .build();

    clock_main_box.append(&date);

//  =========> WEATHER <=========
    // let wh = weather::CurrentWeather::new_example();
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

    // Update the ui state
    let (sender, reciver) = async_channel::bounded::<Option<CurrentWeather>>(1);
    glib::spawn_future_local(glib::clone!(
        #[weak] current_weather_emoji,
        #[weak] current_weather_temp,
        #[weak] current_weather_feels_like,
        #[weak] current_weather_humidity,
        #[weak] current_weather_prec,
        #[weak] current_weather_wind,
        #[weak] current_weather_string,
        async move {
            while let Ok(wh_opt) = reciver.recv().await {
                if let Some(wh) = wh_opt {
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
            }
        }
    ));
    // =======================

    // Button to update the current weather
    let update_button = Button::with_label("Update weather");
    update_button.connect_clicked(move |_| {
        glib::spawn_future_local(glib::clone!(
            #[strong]
            sender,
            async move {
                let data = weather::get_current_weather().await;
                println!("Got current weather");
                sender.send(data).await.expect("Tried to send weather data on async channel");
            }
        ));
    });

    // ROOT of WEATHER
    let weather_box = Gbox::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_start(5)
        .margin_end(5)
        .margin_top(10)
        .build();

    weather_box.append(&current_weather);
    weather_box.append(&update_button);

//  =========> ROOT <=========
    // Main box, root of the app
    let main_box = Gbox::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    main_box.append(&clock_main_box);
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

    window.present();
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
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}