use super::{
    LoadDocument,
    SetStatus,
    UpdatePreview,
};
use crate::{
    document_window::DocumentWindow,
    util::get_dialog_path,
};
use gtk4::{
    glib::clone,
    traits::{
        DialogExt,
        GtkWindowExt,
        WidgetExt,
    },
    FileChooserAction,
    FileChooserDialog,
    ResponseType,
};

#[must_use]
pub struct PromptLoadDocument<'a>
{
    pub win: &'a DocumentWindow,
}

impl PromptLoadDocument<'_>
{
    pub fn invoke(self)
    {
        let load_document_dialog = FileChooserDialog::new(
            Some("Load GlyphMosaic Document"),
            Some(self.win),
            FileChooserAction::Open,
            &[
                ("Open", ResponseType::Ok),
                ("Cancel", ResponseType::Cancel),
            ],
        );

        load_document_dialog.connect_response(
            clone!(@strong self.win as win =>
                move |dialog, response| {
                    handle_load_document_dialog_response(&win,dialog,response);
                }
            ),
        );

        load_document_dialog.show();
    }
}

fn handle_load_document_dialog_response(
    win: &DocumentWindow,
    dialog: &FileChooserDialog,
    response: ResponseType,
)
{
    dialog.close();

    if response != ResponseType::Ok
    {
        return;
    }

    let result: Result<(), String> = try {
        let file_path = get_dialog_path(dialog)?;

        LoadDocument {
            load_from_path: file_path,
            win,
        }
        .invoke();
    };

    if let Err(e) = result
    {
        let message =
            format!("Error loading document: {e}");

        SetStatus { message, win }.invoke();
    };

    UpdatePreview { win }.invoke();
}
