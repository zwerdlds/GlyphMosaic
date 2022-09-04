use crate::document_window::DocumentWindow;
use gtk4::subclass::prelude::ObjectSubclassIsExt;

#[must_use]
pub struct WindowDocumentCommand<'a>
{
    pub command: glyph_mosaic::commands::DocumentCommand,
    pub win: &'a DocumentWindow,
}

impl WindowDocumentCommand<'_>
{
    pub fn invoke(self)
    {
        self.win
            .imp()
            .model
            .borrow_mut()
            .apply_command(self.command);
    }
}
