use gtk4::{
    subclass::prelude::ObjectSubclassIsExt,
    traits::AdjustmentExt,
};

use crate::document_window::{
    drawing_area_point::DrawingAreaPoint,
    DocumentWindow,
};

use super::PaintCoords;

#[must_use]
pub struct StartDrag<'a>
{
    pub win: &'a DocumentWindow,
    pub pt: DrawingAreaPoint,
}

impl StartDrag<'_>
{
    pub fn invoke(self)
    {
        let dp = self
            .pt
            .as_document_point(self.win.imp().zoom.value());

        self.win
            .imp()
            .model
            .borrow_mut()
            .set_last_drag_pos(Some(dp));

        let pts = dp.into();

        PaintCoords { win: self.win, pts }.invoke();
    }
}
