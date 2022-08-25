use super::DocumentWindow;
use glyph_mosaic::prelude::*;
use gtk4::{
    gdk_pixbuf::Pixbuf,
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
    FileChooserAction,
    FileChooserDialog,
    ResponseType,
};

impl DocumentWindow
{
    pub fn setup_events(&self)
    {
        self.setup_base_image_select();
        self.setup_preview_redraw();
    }

    fn setup_base_image_select(&self)
    {
        let gtk_window = self.clone();
        let win = self.imp();
        win.select_source.connect_clicked(move |_| {
            gtk_window.select_source();
        });
    }

    pub fn setup_preview_redraw(&self)
    {
        let gtk_window = self.clone();
        let win = self.imp();
        win.preview_area.set_draw_func(
            move |_area, ctx, _width, _height| {
                let res: Result<(), String> = try {
                    let preview: Pixbuf = gtk_window
                        .imp()
                        .document
                        .borrow()
                        .create_preview()?;

                    ctx.set_source_pixbuf(
                        &preview, 0.0, 0.0,
                    );

                    ctx.paint().map_err(|_| {
                        "Invalid cairo surface state."
                    })?;
                };

                if let Err(e) = res
                {
                    gtk_window.imp().set_status(
                        format!(
                            "Error painting preview: {e}"
                        )
                        .as_str(),
                    );
                }
            },
        );
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
                    let result: Result<String, String> = try {
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
