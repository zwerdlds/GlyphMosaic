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

pub struct PromptLoadSourceImage
{
    win: DocumentWindow,
}

impl PromptLoadSourceImage
{
    pub fn new(win: DocumentWindow)
        -> PromptLoadSourceImage
    {
        PromptLoadSourceImage { win }
    }

    pub fn invoke(self)
    {
        let load_source_dialog = FileChooserDialog::new(
            Some("Select Source Image"),
            Some(&self.win),
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
                HandleLoadSourceImageResponse::new(dialog.to_owned(),response,win.clone()).invoke();
              }),
        );

        load_source_dialog.show();
    }
}
