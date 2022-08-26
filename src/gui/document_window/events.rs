use super::DocumentWindow;
use glyph_mosaic::prelude::DocumentPropertied;
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
            gtk_window.select_base_image();
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
                        .model
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

    fn select_base_image(&self)
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
                    d.close();

                    let result: Result<String, String> =
                        try {
                            let file_path = d
                                .file()
                                .ok_or(
                                    "Couldn't get file \
                                     from filechooser.",
                                )?
                                .path()
                                .ok_or(
                                    "Couldn't get file \
                                     path.",
                                )?
                                .to_str()
                                .ok_or(
                                    "Path not convertable \
                                     to string.",
                                )?
                                .to_owned();

                            let img = Pixbuf::from_file(
                                    file_path.clone(),
                                )
                                .map_err(|e| {
                                    format!(
                                        "Failed getting image \
                                        data: {e}"
                                    )
                                })?;

                                
                            win.imp()
                                .model
                                .borrow_mut()
                                .set_base_image(img.into());

                            format!("Loaded file from {file_path}")
                        };

                    let result = result
                        .map_err(
                            |e| {
                                format!(
                                    "Loading base image \
                                     failed: {e}"
                                )
                            },
                        );

                    win.imp().set_status_from_res(result);
                    win.imp().refresh_preview();
                }
            },
        );

        load_source_dialog.show();
    }
}
