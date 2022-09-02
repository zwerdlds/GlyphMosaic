use super::{
    PaintCoords,
    SetStatus,
    WindowCommand,
};
use crate::document_window::{
    drawing_area_point::DrawingAreaPoint,
    DocumentWindow,
};
use delegate::delegate;
use glyph_mosaic::{
    document::DocumentPoint,
    point::GenericPoint,
};
use gtk4::{
    subclass::prelude::ObjectSubclassIsExt,
    traits::{
        AdjustmentExt,
        GestureDragExt,
    },
    GestureDrag,
};

pub struct DragRelativePoint(GenericPoint<f64>);
impl DragRelativePoint
{
    delegate! {
        to self.0 {
            pub fn x(&self) -> f64;
            pub fn y(&self) -> f64;
        }
    }

    pub fn new(pt: (f64, f64)) -> Self
    {
        Self(pt.into())
    }

    pub fn as_non_relative_point(
        &self,
        relative_to: DrawingAreaPoint,
    ) -> DrawingAreaPoint
    {
        (
            self.x() + relative_to.x(),
            self.y() + relative_to.y(),
        )
            .into()
    }
}
impl From<(f64, f64)> for DragRelativePoint
{
    fn from(pt: (f64, f64)) -> Self
    {
        Self::new(pt)
    }
}

pub struct UpdateDrag
{
    win: DocumentWindow,
    dg: GestureDrag,
    pt: DragRelativePoint,
}

impl UpdateDrag
{
    pub fn new(
        win: DocumentWindow,
        dg: GestureDrag,
        pt: DragRelativePoint,
    ) -> UpdateDrag
    {
        UpdateDrag { win, dg, pt }
    }
}

impl WindowCommand for UpdateDrag
{
    fn invoke(self)
    {
        let res = try {
            let from = self
                .win
                .imp()
                .model
                .borrow()
                .last_drag_pos()
                .ok_or("Dragged with no start point.")?;

            let drag_relative_pt: DrawingAreaPoint = self
                .dg
                .start_point()
                .ok_or("Drag start point not specified")?
                .into();

            let new_drag_location = self
                .pt
                .as_non_relative_point(drag_relative_pt);

            self.win
                .imp()
                .model
                .borrow_mut()
                .set_last_drag_pos(Some(
                    new_drag_location.into(),
                ));

            let end = new_drag_location.as_document_point(
                self.win.imp().zoom.value(),
            );

            let start: DocumentPoint = from.into();
            let pts = start.raster_line_to(end);

            PaintCoords::new(self.win.clone(), pts)
                .invoke();
        };

        SetStatus::maybe_new_from_res(res, self.win)
            .invoke();
    }
}
