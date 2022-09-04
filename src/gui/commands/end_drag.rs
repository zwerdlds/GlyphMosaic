use crate::document_window::DocumentWindow;
use gtk4::subclass::prelude::ObjectSubclassIsExt;

#[must_use]
pub struct EndDrag<'a>
{
    pub win: &'a DocumentWindow,
}

impl EndDrag<'_>
{
    pub fn invoke(self)
    {
        self.win
            .imp()
            .model
            .borrow_mut()
            .set_last_drag_pos(None);
    }
}
