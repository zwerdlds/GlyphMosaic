use crate::document_window::DocumentWindow;
use gtk4::subclass::prelude::ObjectSubclassIsExt;

pub struct SetStatus
{
    message: String,
    win: DocumentWindow,
}

impl SetStatus
{
    pub fn new(
        message: String,
        win: DocumentWindow,
    ) -> Self
    {
        Self { message, win }
    }

    pub fn new_from_result(
        res: Result<String, String>,
        win: DocumentWindow,
    ) -> Self
    {
        match res
        {
            Ok(msg) => Self::new(msg, win),
            Err(e) => Self::new_error(e, win),
        }
    }

    pub fn new_error(
        e: String,
        win: DocumentWindow,
    ) -> SetStatus
    {
        Self::new(format!("Error: {e}"), win)
    }

    pub fn invoke(self)
    {
        self.win
            .imp()
            .status_label
            .set_label(&self.message);
    }
}
