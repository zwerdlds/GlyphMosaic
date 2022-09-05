use crate::{
    commands::QueuePreviewRefresh,
    document_window::DocumentWindow,
};
use gtk4::{
    self,
    gdk_pixbuf::Pixbuf,
    subclass::prelude::ObjectSubclassIsExt,
};

#[must_use]
pub struct HandlePreviewResult<'a>
{
    pub win: &'a DocumentWindow,
    pub img: Pixbuf,
}

impl HandlePreviewResult<'_>
{
    pub fn invoke(self)
    {
        self.win
            .imp()
            .model
            .borrow_mut()
            .set_current_preview(Some(self.img));

        QueuePreviewRefresh { win: &self.win }.invoke();
    }
}
