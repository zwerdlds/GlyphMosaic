use super::SetStatus;
use crate::{
    document_window::DocumentWindow,
    util::get_dialog_path,
};
use gtk4::{
    glib::clone,
    subclass::prelude::ObjectSubclassIsExt,
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
pub struct SaveDocument<'a>
{
    pub win: &'a DocumentWindow,
}

impl SaveDocument<'_>
{
    pub fn invoke(self)
    {
        let result: Result<(), String> = try {
            if let Some(path) = self
                .win
                .imp()
                .model
                .borrow()
                .document_path()
                .to_owned()
            {
                self.win
                    .imp()
                    .model
                    .borrow()
                    .document()
                    .write_to_location(&path)?;

                SetStatus {
                    message: format!(
                        "File saved to {path}"
                    ),
                    win: self.win,
                }
                .invoke();
            }
            else
            {
                let save_document_dialog =
                    FileChooserDialog::new(
                        Some("Save document"),
                        Some(self.win),
                        FileChooserAction::Save,
                        &[
                            ("Save", ResponseType::Ok),
                            (
                                "Cancel",
                                ResponseType::Cancel,
                            ),
                        ],
                    );

                save_document_dialog.connect_response(
                    clone!(@strong self.win as win =>
                        move |dialog, response| {
                            handle_save_document_dialog_response(&win,dialog,response);
                        }
                    ),
                );

                save_document_dialog.show();
            }
        };

        if let Err(e) = result
        {
            let message =
                format!("Error saving document: {e}");

            SetStatus {
                message,
                win: self.win,
            }
            .invoke();
        };
    }
}

fn handle_save_document_dialog_response(
    win: &DocumentWindow,
    dialog: &FileChooserDialog,
    response: ResponseType,
)
{
    dialog.close();

    if (response != ResponseType::Ok)
        && (response != ResponseType::DeleteEvent)
    {
        return;
    }

    let result: Result<(), String> = try {
        let file_path = get_dialog_path(dialog)?;

        win.imp()
            .model
            .borrow()
            .document()
            .write_to_location(&file_path)?;

        win.imp()
            .model
            .borrow_mut()
            .set_document_path(Some(file_path));
    };

    if let Err(e) = result
    {
        let message =
            format!("Error loading regions map image: {e}");
        SetStatus { message, win }.invoke();
    };
}
