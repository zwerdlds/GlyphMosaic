use super::{
    QueuePreviewRefresh,
    SetStatus,
};
use crate::{
    commands::WindowDocumentCommand,
    document_window::DocumentWindow,
    util::get_dialog_path,
};
use glyph_mosaic::commands::DocumentCommand;
use gtk4::{
    traits::GtkWindowExt,
    FileChooserDialog,
    ResponseType,
};
use std::fs;

#[must_use]
pub struct HandleLoadSourceTextResponse<'a>
{
    pub dialog: &'a FileChooserDialog,
    pub response: ResponseType,
    pub win: &'a DocumentWindow,
}

impl HandleLoadSourceTextResponse<'_>
{
    pub fn invoke(self)
    {
        self.dialog.close();

        if self.response != ResponseType::Ok
        {
            return;
        }

        let result: Result<String, String> = try {
            let file_path = get_dialog_path(&self.dialog)?;

            let txt = fs::read_to_string(file_path.clone())
                .map_err(|e| e.to_string())?;

            WindowDocumentCommand {
                command: DocumentCommand::SetSourceText(
                    txt.into(),
                ),
                win: self.win,
            }
            .invoke();

            format!("Loaded source text from {file_path}")
        };

        let message = match result
        {
            Ok(m) => m,
            Err(e) =>
            {
                format!("Error loading source text: {e}")
            },
        };

        SetStatus {
            message,
            win: self.win,
        }
        .invoke();
        QueuePreviewRefresh { win: self.win }.invoke();
    }
}
