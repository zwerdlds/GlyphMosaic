// pub mod commands;
pub mod drawing_area_point;
pub mod events;
pub mod imp;
use glib::Object;
use gtk4::{
    gio,
    glib,
    Application,
};

glib::wrapper! {
    pub struct DocumentWindow(ObjectSubclass<imp::DocumentWindow>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Accessible, gtk4::Buildable,
                    gtk4::ConstraintTarget, gtk4::Native, gtk4::Root, gtk4::ShortcutManager;
}

impl DocumentWindow
{
    pub fn new(app: &Application) -> Self
    {
        let win: Self =
            Object::new(&[("application", app)])
                .expect("Failed to create Window");

        win.setup_events();

        win
    }
}
