use super::DocumentWindow;
use crate::model::SettingsTab;
use glyph_mosaic::commands::{
    SetSourceImage,
    SetSourceText,
};
use gtk4::{
    cairo::Context,
    gdk_pixbuf::{
        InterpType,
        Pixbuf,
    },
    glib::clone,
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
    DrawingArea,
    FileChooserAction,
    FileChooserDialog,
    ResponseType,
};
use std::fs::read_to_string;

impl DocumentWindow
{
    pub fn setup_events(&self)
    {
        self.setup_source_image_select();
        self.setup_source_text_select();
        self.setup_preview_redraw();
        self.setup_zoom();
        self.setup_preview_opacity();
        self.setup_settings_tabs();
        self.setup_drawing();
    }

    fn setup_settings_tabs(&self)
    {
        self.imp().settings_notebook.connect_switch_page(
            clone!(@strong self as win => move |_,_,pg|{
                win.update_settings_tab(pg);
            }),
        );
    }

    fn update_settings_tab(
        &self,
        page_index: u32,
    )
    {
        use SettingsTab::*;

        let p: Result<SettingsTab, String> =
            match page_index
            {
                0 => Ok(Sources),
                1 => Ok(Regions),
                2 => Ok(Lines),
                3 => Ok(Points),
                4 => Ok(Glyphs),
                5 => Ok(Export),
                i =>
                {
                    Err(format!(
                        "Tab index {i} is not supported."
                    ))
                },
            };

        match p
        {
            Ok(p) =>
            {
                self.imp()
                    .model
                    .borrow_mut()
                    .set_settings_tab(p);
                self.imp().queue_preview_refresh();
            },
            Err(e) =>
            {
                self.imp().set_status(
                    format!("Error switching tab: {e}")
                        .as_str(),
                )
            },
        }
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

        if response != ResponseType::Ok
        {
            return;
        }

        let result: Result<String, String> = try {
            let file_path = Self::get_dialog_path(dialog)?;

            let img = Pixbuf::from_file(file_path.clone())
                .map_err(|e| {
                    format!(
                        "Failed getting source image \
                         data: {e}"
                    )
                })?;

            self.apply_command(SetSourceImage {
                source_image: Some(img),
            });

            format!("Loaded source image from {file_path}")
        };

        let result = result.map_err(|e| {
            format!("Loading source image failed: {e}")
        });

        self.set_status_from_res(result);
        self.queue_preview_refresh();
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

        if response != ResponseType::Ok
        {
            return;
        }

        let result: Result<String, String> = try {
            let file_path = Self::get_dialog_path(dialog)?;

            let txt = read_to_string(file_path.clone())
                .map_err(|e| e.to_string())?;

            self.imp().apply_command(SetSourceText {
                source_text: Some(txt),
            });

            format!("Loaded source text from {file_path}")
        };

        let result = result.map_err(|e| {
            format!("Loading source text failed: {e}")
        });

        self.set_status_from_res(result);
        self.queue_preview_refresh();
    }

    fn setup_preview_opacity(&self)
    {
        self.imp().preview_opacity.connect_value_changed(
            clone!(@strong self as win => move |_| {
                win.queue_preview_refresh();
            }),
        );
    }

    fn setup_zoom(&self)
    {
        self.imp().zoom.connect_value_changed(
            clone!(@strong self as win => move |_| {
                win.queue_preview_refresh();
            }),
        );
    }

    fn setup_preview_redraw(&self)
    {
        self.imp().preview_area.set_draw_func(
            clone!(@strong self as win => move |area, ctx, _width, _height| {
                win.redraw_preview(area, ctx);
            })
        );
    }

    fn redraw_preview(
        &self,
        area: &DrawingArea,
        ctx: &Context,
    )
    {
        let win = self.imp();

        let res: Result<(), String> = try {
            let model = win.model.borrow();
            let zoom = win.zoom.value();

            let unproc_preview = model.create_preview()?;

            let w = (unproc_preview.width() as f64 * zoom)
                as i32;
            let h = (unproc_preview.height() as f64 * zoom)
                as i32;

            area.set_width_request(w);
            area.set_height_request(h);

            let preview = {
                let p = unproc_preview
                    .scale_simple(
                        w,
                        h,
                        InterpType::Bilinear,
                    )
                    .ok_or(format!(
                        "Scaling preview result failed."
                    ))?
                    .clone();

                let preview_opacity =
                    win.preview_opacity.value();

                model
                    .create_source_preview_base()?
                    .scale_simple(
                        w,
                        h,
                        InterpType::Bilinear,
                    )
                    .ok_or(format!(
                        "Scaling source for underlay \
                         failed."
                    ))?
                    .composite(
                        &p,
                        0,
                        0,
                        w,
                        h,
                        0f64,
                        0f64,
                        1f64,
                        1f64,
                        InterpType::Bilinear,
                        255 - (255 as f64 * preview_opacity)
                            as i32,
                    );

                p
            };

            ctx.set_source_pixbuf(&preview, 0f64, 0f64);

            ctx.paint().map_err(|_| {
                "Invalid cairo surface state."
            })?;
        };

        let res = res.map_err(|e| {
            format!("Getting preview failed: {e}")
        });

        self.maybe_set_error_from_res(res);
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
