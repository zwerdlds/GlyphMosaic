use super::SetStatus;
use crate::document_window::DocumentWindow;
use gtk4::{
    self,
    cairo::Context,
    gdk_pixbuf::Pixbuf,
    prelude::GdkCairoContextExt,
    subclass::prelude::ObjectSubclassIsExt,
    traits::WidgetExt,
    DrawingArea,
};

#[must_use]
pub struct RedrawPreview<'a>
{
    pub area: &'a DrawingArea,
    pub win: &'a DocumentWindow,
    pub ctx: &'a Context,
}

impl RedrawPreview<'_>
{
    pub fn invoke(self)
    {
        let res: Result<(), String> = try {
            let model = self.win.imp().model.borrow();
            let img: &Pixbuf = model
                .current_preview()
                .as_ref()
                .ok_or("No picture to refresh.")?;

            let w = img.width();
            let h = img.height();
            self.area.set_width_request(w);
            self.area.set_height_request(h);

            self.ctx.save().map_err(|e| {
                format!("Cairo context save failed ({e})")
            })?;

            self.ctx.set_source_pixbuf(&img, 0f64, 0f64);

            self.ctx.paint().map_err(|e| {
                format!("Invalid cairo surface state ({e})")
            })?;

            self.ctx.restore().map_err(|e| {
                format!("Cairo context save failed ({e})")
            })?;
        };

        if let Err(msg) = res
        {
            SetStatus {
                message: format!(
                    "Drawing preview failed: {msg}"
                ),
                win: self.win,
            }
            .invoke();
        }
    }
}
