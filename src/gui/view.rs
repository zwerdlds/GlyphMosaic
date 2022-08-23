use gtk4::glib;
use gtk4::Builder;

#[derive(Clone)]
pub struct View {
    gtk_ui_builder: Builder,
}

impl View {
    pub fn new() -> View {
        print!("Loading GUI");
        let glade_src = include_str!("glyphmosaicgui.ui");
        let gtk_ui_builder = Builder::new();
        gtk_ui_builder.add_from_string(glade_src).unwrap();

        View { gtk_ui_builder }
    }

    fn get_object<T>(
        &self,
        object_id: &str,
    ) -> T
    where
        T: glib::IsA<glib::Object>,
    {
        self.try_get_object(object_id).expect(&format!(
            "Getting object with id {object_id} failed.",
        ))
    }

    fn try_get_object<T>(
        &self,
        object_id: &str,
    ) -> Option<T>
    where
        T: glib::IsA<glib::Object>,
    {
        self.gtk_ui_builder.object(object_id)
    }

    pub fn main_window(&self) -> gtk4::ApplicationWindow {
        self.get_object("GMMainWindow")
    }
}
