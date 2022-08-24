use gtk4::{
    glib::clone,
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
    FileChooserAction,
    FileChooserNative,
    ResponseType,
};

use super::Window;

impl Window
{
    pub fn setup_events(&self)
    {
        let imp = self.imp();
        let window = self.clone();

        imp.select_source.connect_clicked(clone!(
            @strong window =>
            move |_| {
                window.select_source();
            }
        ));

        // self.settings_notebook.connect_open(move |_, i| {
        //     print!("Settings page changing ({i}).");
        //     true
        // });
    }

    fn select_source(&self)
    {
        let load_source_dialog = FileChooserNative::new(
            Some("Select Source Image"),
            Some(self),
            FileChooserAction::Open,
            Some("Open"),
            Some("Cancel"),
        );

        load_source_dialog.connect_response(
            move |d: &FileChooserNative,
                  response: ResponseType| {
                if response == ResponseType::Ok
                {
                    let file = d
                        .file()
                        .expect("Couldn't get file");

                    let file_name = file
                        .path()
                        .expect("Couldn't get file path");

                    let file_name =
                        file_name.to_str().unwrap();

                    print!("{file_name}");
                }
            },
        );

        load_source_dialog.show();
    }
}
