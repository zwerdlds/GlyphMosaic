mod window;
use gtk4::{
    gio,
    prelude::*,
};
use window::Window;

const APP_ID: &str = "me.zwerdlds.glyphmosaic";

fn main()
{
    gio::resources_register_include!(
        "glyphmosaicgui.gresource"
    )
    .expect("Failed to register resources.");

    let app = gtk4::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        let window = Window::new(app);
        window.setup_events();
        window.present();
        print!("ran app!");
    });

    app.run();
}
