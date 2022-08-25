use gtk4::{
    glib::clone,
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
    FileChooserAction,
    FileChooserDialog,
    ResponseType,
};

use super::DocumentWindow;

impl DocumentWindow
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
    }

    fn select_source(&self)
    {
        let win: DocumentWindow = self.clone();

        let load_source_dialog = FileChooserDialog::new(
            Some("Select Source Image"),
            Some(self),
            FileChooserAction::Open,
            &[
                ("Open", ResponseType::Ok),
                ("Cancel", ResponseType::Cancel),
            ],
        );

        load_source_dialog.connect_response(
            move |d: &FileChooserDialog,
                  response: ResponseType| {
                if response == ResponseType::Ok
                {
                    let result: Result<String, &str> = try {
                        let file = d
                            .file()
                            .ok_or("Couldn't get file")?;

                        let file_name = file.path().ok_or(
                            "Couldn't get file path",
                        )?;

                        file_name
                            .to_str()
                            .ok_or(
                                "Path not convertable to \
                                 string.",
                            )?
                            .to_owned()
                    };
                    d.close();

                    let res = match result.as_ref()
                    {
                        Ok(p) =>
                        {
                            win.imp()
                                .document
                                .borrow_mut()
                                .set_source_path(
                                    p.to_string(),
                                );
                            format!(
                                "Base image loaded from \
                                 {p}."
                            )
                        },
                        Err(e) =>
                        {
                            format!(
                                "Error loading file: {e}"
                            )
                        },
                    };

                    win.imp().set_status(&res);
                    win.imp().refresh_preview();
                }
            },
        );

        load_source_dialog.show();
    }
}
