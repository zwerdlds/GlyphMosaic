use crate::{
    commands::HandleLoadSourceTextResponse,
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

pub struct PromptLoadSourceText
{
    win: DocumentWindow,
}

impl PromptLoadSourceText
{
    pub fn new(win: DocumentWindow)
        -> PromptLoadSourceText
    {
        PromptLoadSourceText { win }
    }

    pub fn invoke(self)
    {
        let load_source_dialog = FileChooserDialog::new(
            Some("Select Source Text"),
            Some(&self.win),
            FileChooserAction::Open,
            &[
                ("Open", ResponseType::Ok),
                ("Cancel", ResponseType::Cancel),
            ],
        );

        load_source_dialog.connect_response(
            clone!(@strong self.win as win =>
            move |d: &FileChooserDialog,
              response: ResponseType| {
                HandleLoadSourceTextResponse::new(d.to_owned(),response,win.clone()).invoke();
              }),
        );

        load_source_dialog.show();
    }
}
