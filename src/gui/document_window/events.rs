use super::DocumentWindow;
use delegate::delegate;
use glyph_mosaic::prelude::DocumentPropertied;
use gtk4::{
    gdk_pixbuf::{
        InterpType,
        Pixbuf,
    },
    glib::clone,
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
    FileChooserAction,
    FileChooserDialog,
    ResponseType,
};
use std::fs::read_to_string;

impl DocumentWindow
{
    delegate! {
        to self.imp()
        .model
        .borrow_mut() {
            fn set_source_image(&self, image: Option<Pixbuf>);
            fn set_source_text(&self, text: Option<String>);
        }
    }

    pub fn setup_events(&self)
    {
        self.setup_source_image_select();
        self.setup_source_text_select();
        self.setup_preview_redraw();
        self.setup_zoom();
    }

    fn setup_source_image_select(&self)
    {
        self.imp().select_source_image.connect_clicked(
            clone!(@strong self as win => move |_| {
                win.prompt_load_source_image();
            }),
        );
    }

    fn prompt_load_source_image(&self)
    {
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
            clone!(@strong self as win =>
            move |d: &FileChooserDialog,
              response: ResponseType| {
                win.handle_load_source_image_response(d,response);
              }),
        );

        load_source_dialog.show();
    }

    fn handle_load_source_image_response(
        &self,
        dialog: &FileChooserDialog,
        response: ResponseType,
    )
    {
        dialog.close();

        if response == ResponseType::Ok
        {
            let result: Result<String, String> = try {
                let file_path =
                    Self::get_dialog_path(dialog)?;

                let img =
                    Pixbuf::from_file(file_path.clone())
                        .map_err(|e| {
                            format!(
                                "Failed getting source \
                                 image data: {e}"
                            )
                        })?;

                self.set_source_image(Some(img));

                format!(
                    "Loaded source image from {file_path}"
                )
            };

            let result = result.map_err(|e| {
                format!("Loading source image failed: {e}")
            });

            self.imp().set_status_from_res(result);
            self.imp().queue_preview_refresh();
        }
    }

    fn setup_source_text_select(&self)
    {
        self.imp().select_source_text.connect_clicked(
            clone!(@strong self as win => move |_| {
                win.prompt_load_source_text();
            }),
        );
    }

    fn prompt_load_source_text(&self)
    {
        let load_source_dialog = FileChooserDialog::new(
            Some("Select Source Text"),
            Some(self),
            FileChooserAction::Open,
            &[
                ("Open", ResponseType::Ok),
                ("Cancel", ResponseType::Cancel),
            ],
        );

        load_source_dialog.connect_response(
            clone!(@strong self as win =>
            move |d: &FileChooserDialog,
              response: ResponseType| {
                win.handle_load_source_text_response(d,response);
              }),
        );

        load_source_dialog.show();
    }

    fn get_dialog_path(
        dialog: &FileChooserDialog
    ) -> Result<String, String>
    {
        dialog
            .file()
            .ok_or("Couldn't get file from filechooser.")?
            .path()
            .ok_or("Couldn't get file path.")?
            .to_str()
            .ok_or("Path not convertable to string.")
            .map(|p| p.to_string())
            .map_err(|e| e.to_string())
    }

    fn handle_load_source_text_response(
        &self,
        dialog: &FileChooserDialog,
        response: ResponseType,
    )
    {
        dialog.close();

        if response == ResponseType::Ok
        {
            let result: Result<String, String> = try {
                let file_path =
                    Self::get_dialog_path(dialog)?;

                let txt = read_to_string(file_path.clone())
                    .map_err(|e| e.to_string())?;

                self.set_source_text(Some(txt));

                format!(
                    "Loaded source text from {file_path}"
                )
            };

            let result = result.map_err(|e| {
                format!("Loading source text failed: {e}")
            });

            self.imp().set_status_from_res(result);
            self.imp().queue_preview_refresh();
        }
    }

    fn setup_zoom(&self)
    {
        self.imp().zoom.connect_value_changed(
            clone!(@strong self as win => move |_| {
                win.imp().queue_preview_refresh();
            }),
        );
    }

    fn setup_preview_redraw(&self)
    {
        self.imp().preview_area.set_draw_func(
            clone!(@strong self as win => move |area, ctx, _width, _height| {
                let win = win.imp();

                let res: Result<(), String> = try {
                    let model = win.model.borrow();

                    let zoom = win.zoom.value();

                    let unproc_preview =
                        model.create_preview()?;

                    let w = (unproc_preview.width() as f64
                        * zoom)
                        as i32;
                    let h = (unproc_preview.height() as f64
                        * zoom)
                        as i32;

                    area.set_width_request(w);
                    area.set_height_request(h);

                    let preview = unproc_preview
                        .scale_simple(
                            w,
                            h,
                            InterpType::Bilinear,
                        )
                        .ok_or(format!("Zoom failed."))?;

                    ctx.set_source_pixbuf(
                        &preview, 0f64, 0f64,
                    );

                    ctx.paint().map_err(|_| {
                        "Invalid cairo surface state."
                    })?;
                };

                if let Err(e) = res
                {
                    win.set_status(
                        format!(
                            "Getting preview encountered \
                             error: {e}"
                        )
                        .as_str(),
                    );
                }
            })
        );
    }
}

#[cfg(test)]
mod tests
{
    use std::path::{
        Path,
        PathBuf,
    };

    #[test]
    fn validate_path_canonicalization()
    {
        let path = Path::new("./Cargo.toml");
        let canon = path.canonicalize().unwrap();

        assert_eq!(
            canon,
            PathBuf::from(
                "/mnt/Speedy/Development/GlyphMosaic/\
                 Cargo.toml"
            )
        );
    }
}
