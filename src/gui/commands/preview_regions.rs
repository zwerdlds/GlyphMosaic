use super::{
    SetStatus,
    WaitForPreviewResult,
};
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
    gdk_pixbuf::{
        Colorspace,
        InterpType,
        Pixbuf,
    },
    subclass::prelude::ObjectSubclassIsExt,
    traits::AdjustmentExt,
};
use std::collections::HashSet;

#[must_use]
pub struct PreviewRegions<'a>
{
    pub win: &'a DocumentWindow,
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

    pub fn start_background_render(
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
            .to_owned()
            .into();

        let render_handle = self
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
                        (region[0].x() % 255) as u8,
                        (region[0].y() % 255) as u8,
                        ((region[0].x() + region[0].y())
                            % 255)
                            as u8,
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
            });

        Ok(render_handle)
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

        let rh = self.start_background_render(
            zoom,
            preview_opacity,
            region_border_pixels,
        )?;

        WaitForPreviewResult {
            win: self.win.clone(),
            render_handle: rh,
        }
        .invoke();

        Ok(())
    }
}
