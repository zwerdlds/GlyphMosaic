use crate::document_window::DocumentWindow;
use glyph_mosaic::commands::DocumentCommand;
use gtk4::subclass::prelude::ObjectSubclassIsExt;

pub struct WindowDocumentCommand
{
    command: glyph_mosaic::commands::DocumentCommand,
    win: DocumentWindow,
}

impl WindowDocumentCommand
{
    pub fn new(
        command: DocumentCommand,
        win: DocumentWindow,
    ) -> WindowDocumentCommand
    {
        WindowDocumentCommand { command, win }
    }

    pub fn invoke(self)
    {
        self.win
            .imp()
            .model
            .borrow_mut()
            .apply_command(self.command);
    }
}
