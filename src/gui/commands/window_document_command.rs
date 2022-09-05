use crate::document_window::DocumentWindow;
use glyph_mosaic::commands::DocumentTransformable;
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
        let doc = self.command.transform_document(
            self.win.imp().model.borrow().document(),
        );

        self.win.imp().model.borrow_mut().set_document(doc);
    }
}
