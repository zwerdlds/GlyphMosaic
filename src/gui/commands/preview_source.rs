use super::{
    HandlePreviewResult,
    SetStatus,
    WaitForPreviewResult,
};
use crate::document_window::DocumentWindow;
use glyph_mosaic::document::image::PixbufDef;
use gtk4::{
    self,
    gdk_pixbuf::{
        InterpType,
        Pixbuf,
    },
    subclass::prelude::ObjectSubclassIsExt,
    traits::AdjustmentExt,
};

#[must_use]
pub struct PreviewSource<'a>
{
    pub win: &'a DocumentWindow,
    pub run_background: bool,
}

impl PreviewSource<'_>
{
    pub fn invoke(self)
    {
        if self.run_background
        {
            self.render_background();
        }
        else
        {
            self.render_foreground();
        }
    }

    pub fn render_background(&self)
    {
        let res: Result<(), String> = try {
            let zoom = self.win.imp().zoom.value();

            let src_img: PixbufDef = self
                .win
                .imp()
                .model
                .borrow()
                .document()
                .source_image()
                .as_ref()
                .ok_or(
                    "No source image specified."
                        .to_string(),
                )?
                .clone()
                .into();

            let render_handle = self
                .win
                .imp()
                .model
                .borrow_mut()
                .checkout_render(async move {
                    try {
                        PreviewSource::render(
                            src_img.into(),
                            zoom,
                        )?
                        .into()
                    }
                })
                .clone();

            WaitForPreviewResult {
                win: self.win.clone(),
                render_handle,
            }
            .invoke();
        };

        if let Err(msg) = res
        {
            SetStatus {
                message: format!(
                    "Error setting up background source \
                     preview: {msg}"
                ),
                win: self.win,
            }
            .invoke();
        }
    }

    pub fn render_foreground(&self)
    {
        let res: Result<(), String> = try {
            let zoom = self.win.imp().zoom.value();

            let src_img: Pixbuf = self
                .win
                .imp()
                .model
                .borrow()
                .document()
                .source_image()
                .as_ref()
                .ok_or(
                    "No source image specified."
                        .to_string(),
                )?
                .clone()
                .into();

            let img = PreviewSource::render(src_img, zoom)?;

            HandlePreviewResult { win: self.win, img }
                .invoke();
        };

        if let Err(msg) = res
        {
            SetStatus {
                message: format!(
                    "Error setting up background source \
                     preview: {msg}"
                ),
                win: self.win,
            }
            .invoke();
        }
    }

    pub fn render(
        src_img: Pixbuf,
        zoom: f64,
    ) -> Result<Pixbuf, String>
    {
        try {
            let w = (src_img.width() as f64 * zoom) as i32;
            let h = (src_img.height() as f64 * zoom) as i32;

            src_img
                .scale_simple(w, h, InterpType::Nearest)
                .ok_or(format!(
                    "Scaling preview result failed."
                ))?
        }
    }
}
