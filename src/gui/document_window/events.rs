use super::DocumentWindow;
use glyph_mosaic::prelude::DocumentPropertied;
use gtk4::{
    gdk_pixbuf::Pixbuf,
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
    FileChooserAction,
    FileChooserDialog,
    ResponseType, ScrollType,
};

impl DocumentWindow
{
    pub fn setup_events(&self)
    {
        self.setup_base_image_select();
        self.setup_preview_redraw();
        self.setup_preview_scrolling();
        self.setup_zoom();
    }

    fn setup_base_image_select(&self)
    {
        let gtk_window = self.clone();
         self.imp().select_source.connect_clicked(move |_| {
            gtk_window.select_base_image();
        });
    }

    fn setup_zoom(&self){
        let win = self.clone();
self.imp().zoom_spinner.connect_changed(move|_|{
    win.imp().queue_preview_refresh();
});
    }


    fn setup_preview_scrolling(&self){
        let win = self.clone();
        self.imp().preview_scroll.connect_scroll_child(move |_scroll,scroll_type, _horizontal|{
            let win = win.imp();
 
            let d: (Option<f32>, Option<f32>) = match scroll_type {
                ScrollType::StepUp => (None,Some(-15f32)),
                ScrollType::StepDown => (None,Some(15f32)),
                ScrollType::StepLeft => (Some(-15f32),None),
                ScrollType::StepRight=> (Some(45f32),None),
                ScrollType::PageUp=> (None,Some(-45f32)),
                ScrollType::PageDown=> (None,Some(45f32)),
                ScrollType::PageLeft=> (Some(-45f32),None),
                ScrollType::PageRight => (Some(45f32),None),
                _ => (None,None),
            };

            if (d.0 != None) || (d.1 != None) {
                let mut p = win.model.borrow_mut().image_center_offset;

                match d.0 {
                Some(dx) => {
                    p.set_x(p.x() + dx);
                    },None=>{}}
                    match d.1 {
                        Some(dy) =>
                {    p.set_x(p.y() + dy);
                }
                None => {}
                }
            }

            win.queue_preview_refresh();

            true
        });
    }

    fn setup_preview_redraw(&self)
    {
        let win = self.clone();
        self.imp().preview_area.set_draw_func(
            move |_area, ctx, _width, _height| {
                let win = win.imp();

                let res: Result<(), String> = try {
                    let model = win.model.borrow();

                    let preview = model
                        .create_preview()?;

                        let zoom = win.zoom_spinner.value();

                        let preview = preview.scale_simple((preview.width() as f64 * zoom) as i32, (preview.height() as  f64 * zoom) as i32, gtk4::gdk_pixbuf::InterpType::Bilinear).ok_or(|e| format!("Zoom failed."))?;

                    let offset = model.image_center_offset;

                    ctx.set_source_pixbuf(
                        &preview, offset.x() as f64,offset.y()as f64
                    );

                    ctx.paint().map_err(|_| {
                        "Invalid cairo surface state."
                    })?;
                };

                if let Err(e) = res
                {
                    win.set_status(
                        format!(
                            "Painting preview encountered error: {e}"
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
                    let win = win.imp();

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

                                
                            win.model
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

                    win.set_status_from_res(result);
                    win.queue_preview_refresh();
                }
            },
        );

        load_source_dialog.show();
    }
}
