use crate::document_window::DocumentWindow;
use gtk4::subclass::prelude::ObjectSubclassIsExt;

#[must_use]
pub struct SetStatus<'a>
{
    pub message: String,
    pub win: &'a DocumentWindow,
}

impl SetStatus<'_>
{
    pub fn invoke(self)
    {
        self.win
            .imp()
            .status_label
            .set_label(&self.message);
    }
}
