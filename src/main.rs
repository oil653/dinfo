use gtk::{
    glib,
    prelude::*,
    Application, 
    CssProvider, 
    gdk::Display, 
    Align,
    Label, 
    Box as Gbox,
};
use gtk_ls::{self, LayerShell, Layer};

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
    let wh = weather::CurrentWeather::new_example_with_code(96);
    
    // CURRENT WEATHER
    // ========================\\
        // LEFT
    let current_weather_emoji = { 
        Label::builder()
        .label(wh.weather_code.to_emoji(wh.is_day))
        .css_classes(["emoji", "current_weather_title"])
        .justify(gtk::Justification::Left)
        .tooltip_text(format!("Cloud cover is {}%", wh.cloud_cover))
        .build()
    };

    let current_weather_string = { 
        Label::builder()
        .label(wh.weather_code.to_string())
        .name("current_weather_string")
        .css_classes(["text"])
        .justify(gtk::Justification::Left)
        .halign(Align::Start)
        .margin_start(10)
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

    let current_weather_temp_box = {
        Gbox::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(Align::Center)
        .build()
    };

    current_weather_temp_box.append(&current_weather_temp);
    current_weather_temp_box.append(&current_weather_feels_like);

    
    let current_left = { Gbox::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(Align::Start)
        .hexpand(true)
        .build()
    };

    let current_left_top = {
        Gbox::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .height_request(80)
        .halign(Align::Center)
        .build()
    };

    current_left_top.append(&current_weather_emoji);
    current_left_top.append(&current_weather_temp_box);

    current_left.append(&current_left_top);
    current_left.append(&current_weather_string);

    let current_weather= { 
        Gbox::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["island"])
        .hexpand(true)
        .vexpand(false)
        // .height_request(140)
        .build() 
    };

        // RIGHT
    let current_right = {
        Gbox::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(Align::End)
        .hexpand(true)
        .margin_end(10)
        .margin_top(5)
        .spacing(5)
        .build()
    };

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

    current_right.append(&current_weather_humidity);
    current_right.append(&current_weather_prec);
    current_right.append(&current_weather_wind);

    current_weather.append(&current_left);
    current_weather.append(&current_right);
    // =======================//
    // ROOT of WEATHER
    let weather_box = Gbox::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_start(5)
        .margin_end(5)
        .margin_top(10)
        .build();

    weather_box.append(&current_weather);

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