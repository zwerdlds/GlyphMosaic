use super::{
    SetStatus,
    UpdatePreview,
};
use crate::{
    commands::WindowDocumentCommand,
    document_window::DocumentWindow,
    util::get_dialog_path,
};
use glyph_mosaic::commands::DocumentCommand;
use gtk4::{
    gdk_pixbuf::Pixbuf,
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
pub struct PromptLoadSourceImage<'a>
{
    pub win: &'a DocumentWindow,
}

impl PromptLoadSourceImage<'_>
{
    pub fn invoke(self)
    {
        let load_source_dialog = FileChooserDialog::new(
            Some("Select Source Image"),
            Some(self.win),
            FileChooserAction::Open,
            &[
                ("Open", ResponseType::Ok),
                ("Cancel", ResponseType::Cancel),
            ],
        );

        load_source_dialog.connect_response(
            clone!(@strong self.win as win =>
                move |dialog, response| {
                    handle_load_source_image_dialog_response(&win, dialog, response);
                }
            ),
        );

        load_source_dialog.show();
    }
}

fn handle_load_source_image_dialog_response(
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

    let result: Result<String, String> = try {
        let file_path = get_dialog_path(dialog)?;

        let img = Pixbuf::from_file(file_path.clone())
            .map_err(|e| {
                format!(
                    "Failed getting source image data: {e}"
                )
            })?;

        WindowDocumentCommand {
            command: DocumentCommand::SetSourceImage(
                img.into(),
            ),
            win,
        }
        .invoke();

        format!("Loaded source image from {file_path}")
    };

    let message = match result
    {
        Ok(m) => m,
        Err(e) =>
        {
            format!("Error loading source image: {e}")
        },
    };

    SetStatus { message, win }.invoke();
    UpdatePreview { win }.invoke();
}
