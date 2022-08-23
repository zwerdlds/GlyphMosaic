mod window;

use gtk4::gio;
use gtk4::prelude::*;
use gtk4::Application;
use window::Window;

const APP_ID: &str = "me.zwerdlds.glyphmosaic";

fn main() {
    gio::resources_register_include!(
        "glyphmosaicgui.gresource"
    )
    .expect("Failed to register resources.");
    // Create a new application
    let app = gtk4::Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
