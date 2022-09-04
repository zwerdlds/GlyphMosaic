use glyph_mosaic::document::DocumentPoint;
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
pub struct Click<'a>
{
    pub pt: DrawingAreaPoint,
    pub win: &'a DocumentWindow,
}

impl Click<'_>
{
    pub fn invoke(self)
    {
        let zoom: f64 = self.win.imp().zoom.value();
        let pts: Vec<DocumentPoint> =
            self.pt.as_document_point(zoom).into();

        PaintCoords { win: self.win, pts }.invoke();
    }
}
