use gtk4::{
    subclass::prelude::ObjectSubclassIsExt,
    traits::AdjustmentExt,
};

use crate::document_window::{
    drawing_area_point::DrawingAreaPoint,
    DocumentWindow,
};

use super::{
    PaintCoords,
    WindowCommand,
};

pub struct Click
{
    pt: DrawingAreaPoint,
    win: DocumentWindow,
}

impl Click
{
    pub fn new(
        pt: DrawingAreaPoint,
        win: DocumentWindow,
    ) -> Click
    {
        Click { pt, win }
    }
}

impl WindowCommand for Click
{
    fn invoke(self)
    {
        let zoom: f64 = self.win.imp().zoom.value();

        let p = self.pt.as_document_point(zoom);

        PaintCoords::new(self.win, p.into()).invoke();
    }
}
