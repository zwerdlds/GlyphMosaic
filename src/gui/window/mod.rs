pub mod events;
pub mod imp;
use glib::Object;
use gtk4::{
    gio,
    glib,
    Application,
};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Accessible, gtk4::Buildable,
                    gtk4::ConstraintTarget, gtk4::Native, gtk4::Root, gtk4::ShortcutManager;
}

impl Window
{
    pub fn new(app: &Application) -> Self
    {
        // Create new window
        Object::new(&[("application", app)])
            .expect("Failed to create Window")
    }
}
