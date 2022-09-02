use super::{
    QueuePreviewRefresh,
    SetStatus,
    WindowCommand,
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

pub struct HandleLoadSourceTextResponse
{
    dialog: FileChooserDialog,
    response: ResponseType,
    win: DocumentWindow,
}

impl HandleLoadSourceTextResponse
{
    pub fn new(
        dialog: FileChooserDialog,
        response: ResponseType,
        win: DocumentWindow,
    ) -> HandleLoadSourceTextResponse
    {
        HandleLoadSourceTextResponse {
            dialog,
            response,
            win,
        }
    }
}

impl WindowCommand for HandleLoadSourceTextResponse
{
    fn invoke(self)
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

            WindowDocumentCommand::new(
                DocumentCommand::SetSourceText(txt.into()),
                self.win.clone(),
            )
            .invoke();

            format!("Loaded source text from {file_path}")
        };

        let result = result.map_err(|e| {
            format!("Loading source text failed: {e}")
        });

        SetStatus::new_from_result(
            result,
            self.win.clone(),
        )
        .invoke();
        QueuePreviewRefresh::new(self.win).invoke();
    }
}
