use crate::document_window::DocumentWindow;
use gtk4::{
    subclass::prelude::ObjectSubclassIsExt,
    traits::WidgetExt,
};

#[must_use]
pub struct QueuePreviewRefresh<'a>
{
    pub win: &'a DocumentWindow,
}

impl QueuePreviewRefresh<'_>
{
    pub fn invoke(self)
    {
        self.win.imp().preview_area.queue_draw();
    }
}
