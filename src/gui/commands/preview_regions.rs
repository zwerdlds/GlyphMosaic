use crate::{
    document_window::DocumentWindow,
    model::RenderHandle,
};
use glyph_mosaic::document::{
    image::PixbufDef,
    region::DocumentRegion,
    DocumentPoint,
};
use gtk4::{
    self,
    cairo::Context,
    gdk_pixbuf::{
        Colorspace,
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
use std::{
    collections::HashSet,
    time::Duration,
};

use super::SetStatus;

#[must_use]
pub struct PreviewRegions<'a>
{
    pub area: &'a DrawingArea,
    pub win: &'a DocumentWindow,
    pub ctx: &'a Context,
}

impl PreviewRegions<'_>
{
    pub fn invoke(self)
    {
        if let Err(msg) = self.setup_update_preview_image()
        {
            SetStatus {
                message: format!(
                    "Error setting up regions preview: \
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
        preview_opacity: f64,
        region_border_pixels: HashSet<DocumentPoint>,
    ) -> Result<RenderHandle, String>
    {
        let dest_img: PixbufDef = self
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
                let dest_img: Pixbuf = dest_img.into();

                let (w, h) =
                    (dest_img.width(), dest_img.height());

                let draw_on_img = Pixbuf::new(
                    Colorspace::Rgb,
                    true,
                    8,
                    w,
                    h,
                )
                .ok_or(
                    "Could not build regions preview \
                     pixbuf.",
                )?;

                let (w, h) = (w as usize, h as usize);

                for pix in region_border_pixels
                    .iter()
                    .filter(|p| (p.x() > 0) && (p.x() < w))
                    .filter(|p| (p.y() > 0) && (p.y() < h))
                {
                    draw_on_img.put_pixel(
                        pix.x() as u32,
                        pix.y() as u32,
                        255,
                        255,
                        255,
                        255,
                    )
                }

                DocumentRegion::calculate_regions(
                    w as usize,
                    h as usize,
                    region_border_pixels,
                )
                .into_iter()
                .for_each(|region| {
                    let (r, g, b) = (
                        rand::random::<u8>(),
                        rand::random::<u8>(),
                        rand::random::<u8>(),
                    );

                    for pix in region.into_iter()
                    {
                        draw_on_img.put_pixel(
                            pix.x() as u32,
                            pix.y() as u32,
                            r,
                            g,
                            b,
                            255,
                        )
                    }
                });

                let (w, h) = (w as i32, h as i32);

                draw_on_img.composite(
                    &dest_img,
                    0,
                    0,
                    w,
                    h,
                    0f64,
                    0f64,
                    1f64,
                    1f64,
                    InterpType::Nearest,
                    255 - (255 as f64 * preview_opacity)
                        as i32,
                );

                let w =
                    (dest_img.width() as f64 * zoom) as i32;
                let h = (dest_img.height() as f64 * zoom)
                    as i32;

                let dest_img = dest_img
                    .scale_simple(w, h, InterpType::Nearest)
                    .ok_or(format!(
                        "Scaling preview result failed."
                    ))?;

                Ok(dest_img.into())
            })
            .clone())
    }

    fn setup_update_preview_image(
        &self
    ) -> Result<(), String>
    {
        let zoom = self.win.imp().zoom.value();
        let preview_opacity =
            self.win.imp().preview_opacity.value();
        let region_border_pixels = self
            .win
            .imp()
            .model
            .borrow()
            .document()
            .region_border_pixels()
            .to_owned();

        let rh = self.start_compute(
            zoom,
            preview_opacity,
            region_border_pixels,
        )?;
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
