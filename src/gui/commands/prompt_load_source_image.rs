use crate::{
    commands::HandleLoadSourceImageResponse,
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
            move |dialog: &FileChooserDialog,
              response: ResponseType| {
                HandleLoadSourceImageResponse{
                    dialog, response,win:&win}
                    .invoke();
              }),
        );

        load_source_dialog.show();
    }
}
