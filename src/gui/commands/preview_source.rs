use std::time::Duration;

use crate::{
    document_window::DocumentWindow,
    model::RenderHandle,
};
use glyph_mosaic::document::image::PixbufDef;
use gtk4::{
    self,
    cairo::Context,
    gdk_pixbuf::{
        InterpType,
        Pixbuf,
    },
    glib::{self,},
    prelude::{
        Continue,
        GdkCairoContextExt,
    },
    subclass::prelude::ObjectSubclassIsExt,
    traits::{
        AdjustmentExt,
        WidgetExt,
    },
    DrawingArea,
};

use super::SetStatus;

#[must_use]
pub struct PreviewSource<'a>
{
    pub area: &'a DrawingArea,
    pub win: &'a DocumentWindow,
    pub ctx: &'a Context,
}

impl PreviewSource<'_>
{
    pub fn invoke(self)
    {
        if let Err(msg) = self.setup_update_preview_image()
        {
            SetStatus {
                message: format!(
                    "Error setting up source preview: \
                     {msg}"
                ),
                win: self.win,
            }
            .invoke();
        }
    }

    pub fn start_compute(
        &self,
        zoom: f64,
    ) -> Result<RenderHandle, String>
    {
        let src_img: PixbufDef = self
            .win
            .imp()
            .model
            .borrow()
            .document()
            .source_image()
            .as_ref()
            .ok_or(
                "No source image specified.".to_string(),
            )?
            .clone()
            .into();

        Ok(self
            .win
            .imp()
            .model
            .borrow_mut()
            .checkout_render(async move {
                let src_img: Pixbuf = src_img.into();

                let w =
                    (src_img.width() as f64 * zoom) as i32;
                let h =
                    (src_img.height() as f64 * zoom) as i32;

                let src_img = src_img
                    .scale_simple(w, h, InterpType::Nearest)
                    .ok_or(format!(
                        "Scaling preview result failed."
                    ))?;

                let ret: PixbufDef = src_img.into();
                Ok(ret)
            })
            .clone())
    }

    fn setup_update_preview_image(
        &self
    ) -> Result<(), String>
    {
        let zoom = self.win.imp().zoom.value();

        let rh = self.start_compute(zoom)?;
        wait_for_result(
            rh,
            self.win.clone(),
            self.ctx.clone(),
            self.area.clone(),
        );
        Ok(())
    }
}

pub fn wait_for_result(
    rh: RenderHandle,
    win: DocumentWindow,
    ctx: Context,
    area: DrawingArea,
)
{
    glib::timeout_add_local(
        Duration::new(0, 1000000),
        move || {
            println!("Checking for results.");

            if win
                .imp()
                .model
                .borrow()
                .is_current_render(&rh)
            {
                if Some(true)
                    == win
                        .imp()
                        .model
                        .borrow()
                        .is_current_render_finished()
                {
                    let res: Result<(), String> = try {
                        let img: Pixbuf = win
                            .imp()
                            .model
                            .borrow_mut()
                            .block_on_current_render()?
                            .into();

                        let w = img.width();
                        let h = img.height();
                        area.set_width_request(w);
                        area.set_height_request(h);

                        ctx.set_source_pixbuf(
                            &img, 0f64, 0f64,
                        );

                        println!("Painting results.");

                        ctx.paint().map_err(|_| {
                            "Invalid cairo surface state."
                        })?;
                    };

                    if let Err(e) = res
                    {
                        SetStatus {
                            message: format!(
                                "Error building preview: \
                                 {e}"
                            ),
                            win: &win,
                        }
                        .invoke();
                    }

                    Continue(false)
                }
                else
                {
                    Continue(true)
                }
            }
            else
            {
                Continue(false)
            }
        },
    );
}
