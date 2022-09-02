use super::WindowCommand;
use crate::document_window::DocumentWindow;
use gtk4::{
    subclass::prelude::ObjectSubclassIsExt,
    traits::WidgetExt,
};

pub struct QueuePreviewRefresh
{
    win: DocumentWindow,
}

impl QueuePreviewRefresh
{
    pub fn new(win: DocumentWindow) -> QueuePreviewRefresh
    {
        QueuePreviewRefresh { win }
    }
}

impl WindowCommand for QueuePreviewRefresh
{
    fn invoke(self)
    {
        self.win.imp().preview_area.queue_draw();
    }
}
