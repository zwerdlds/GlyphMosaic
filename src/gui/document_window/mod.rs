pub mod drawing;
pub mod events;
pub mod imp;
pub mod util;
use delegate::delegate;
use glib::Object;
use gtk4::{
    gio,
    glib,
    subclass::prelude::ObjectSubclassIsExt,
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
    delegate! {
        to self.imp() {
            fn maybe_set_error_from_res(&self, res: Result<(), String>);
            fn set_status_from_res(&self, res: Result<String, String>);
            fn queue_preview_refresh(&self);
            fn apply_command(&self, cmd: impl glyph_mosaic::commands::DocumentCommand);
        }
    }

    pub fn new(app: &Application) -> Self
    {
        // Create new window
        Object::new(&[("application", app)])
            .expect("Failed to create Window")
    }
}
