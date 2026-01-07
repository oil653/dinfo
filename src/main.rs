use gtk::{
    glib,
    prelude::*,
    Application, 
    CssProvider, 
    gdk::Display, 
    // Align,
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
        .css_classes(["title"])
        .vexpand(false)
        .build();

    clock_main_box.append(&clock);



    let date = Label::builder()
        .label("2026/01/01")
        .name("date")
        .css_classes(["title"])
        .vexpand(false)
        .build();

    clock_main_box.append(&date);

//  =========> WEATHER <=========


    let weather_box = Gbox::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

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
        .default_height(800)
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