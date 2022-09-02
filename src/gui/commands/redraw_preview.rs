use super::{
    SetStatus,
    WindowCommand,
};
use crate::document_window::DocumentWindow;
use gtk4::{
    cairo::Context,
    gdk_pixbuf::InterpType,
    prelude::GdkCairoContextExt,
    subclass::prelude::ObjectSubclassIsExt,
    traits::{
        AdjustmentExt,
        WidgetExt,
    },
    DrawingArea,
};

pub struct RedrawPreview
{
    area: DrawingArea,
    win: DocumentWindow,
    ctx: Context,
}

impl RedrawPreview
{
    pub fn new(
        area: DrawingArea,
        win: DocumentWindow,
        ctx: Context,
    ) -> RedrawPreview
    {
        RedrawPreview { area, win, ctx }
    }
}

impl WindowCommand for RedrawPreview
{
    fn invoke(self)
    {
        let res: Result<(), String> = try {
            let unproc_preview = self
                .win
                .imp()
                .model
                .borrow()
                .create_preview()?;

            let zoom = self.win.imp().zoom.value();
            let w = (unproc_preview.width() as f64 * zoom)
                as i32;
            let h = (unproc_preview.height() as f64 * zoom)
                as i32;

            self.area.set_width_request(w);
            self.area.set_height_request(h);

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
                    self.win.imp().preview_opacity.value();

                self.win
                    .imp()
                    .model
                    .borrow()
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

            self.ctx
                .set_source_pixbuf(&preview, 0f64, 0f64);

            self.ctx.paint().map_err(|_| {
                "Invalid cairo surface state."
            })?;
        };

        let res = res.map_err(|e| {
            format!("Getting preview failed: {e}")
        });

        SetStatus::maybe_new_from_res(res, self.win)
            .invoke();
    }
}
