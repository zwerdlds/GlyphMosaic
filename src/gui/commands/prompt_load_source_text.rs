use crate::{
    util::get_dialog_path,
    document_window::DocumentWindow,
};
use gtk4::{
    glib::clone,
    traits::{
        DialogExt,
        WidgetExt,
    },
    FileChooserAction,
    FileChooserDialog,
    ResponseType,
};
use std::fs;
use gtk4::{
    traits::GtkWindowExt,
};
use super::{
    SetStatus,
    UpdatePreview
};
use glyph_mosaic::commands::DocumentCommand;
use crate::{
    commands::WindowDocumentCommand,};


#[must_use]
pub struct PromptLoadSourceText<'a>
{
    pub win: &'a DocumentWindow,
}

impl PromptLoadSourceText<'_>
{
    pub fn invoke(self)
    {
        let load_source_dialog = FileChooserDialog::new(
            Some("Select Source Text"),
            Some(self.win),
            FileChooserAction::Open,
            &[
                ("Open", ResponseType::Ok),
                ("Cancel", ResponseType::Cancel),
            ],
        );

        load_source_dialog.connect_response(
            clone!(@strong self.win as win =>
            move |dialog: &FileChooserDialog,
              response: ResponseType| {
                dialog.close();

                if response != ResponseType::Ok
                {
                    return;
                }
        
                let result: Result<String, String> = try {
                    let file_path = get_dialog_path(dialog)?;
        
                    let txt = fs::read_to_string(file_path.clone())
                        .map_err(|e| e.to_string())?;
        
                    WindowDocumentCommand {
                        command: DocumentCommand::SetSourceText(
                            txt.into(),
                        ),
                        win: &win,
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
                    win: &win,
                }
                .invoke();
        
                UpdatePreview{win:&win}.invoke();
              }),
        );

        load_source_dialog.show();
    }
}
