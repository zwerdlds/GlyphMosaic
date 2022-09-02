use gtk4::{
    subclass::prelude::ObjectSubclassIsExt,
    traits::AdjustmentExt,
};

use crate::document_window::{
    drawing_area_point::DrawingAreaPoint,
    DocumentWindow,
};

use super::PaintCoords;

pub struct StartDrag
{
    win: DocumentWindow,
    pt: DrawingAreaPoint,
}

impl StartDrag
{
    pub fn new(
        win: DocumentWindow,
        pt: DrawingAreaPoint,
    ) -> StartDrag
    {
        StartDrag { win, pt }
    }

    pub fn invoke(self)
    {
        let dp = self
            .pt
            .as_document_point(self.win.imp().zoom.value());

        self.win
            .imp()
            .model
            .borrow_mut()
            .set_last_drag_pos(dp.into());

        let pts = self
            .pt
            .as_document_point(self.win.imp().zoom.value())
            .into();

        PaintCoords::new(self.win, pts).invoke();
    }
}
