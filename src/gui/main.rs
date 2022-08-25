#![feature(try_blocks)]
mod document_window;
use document_window::DocumentWindow;
use glyph_mosaic::document::Document;
use gtk4::{
    gio,
    prelude::*,
    subclass::prelude::*,
};
use std::cell::RefCell;

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
        let window = DocumentWindow::new(app);
        window.setup_events();
        window
            .imp()
            .document
            .swap(&RefCell::new(Document::new()));
        window.present();
        print!("ran app!");
    });

    app.run();
}
