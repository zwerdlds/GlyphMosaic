use super::WindowCommand;
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

    pub fn maybe_new_from_res(
        res: Result<(), String>,
        win: DocumentWindow,
    ) -> Option<SetStatus>
    {
        match res
        {
            Err(msg) => Some(Self::new_error(msg, win)),
            Ok(()) => None,
        }
    }

    pub fn new_error(
        e: String,
        win: DocumentWindow,
    ) -> SetStatus
    {
        Self::new(format!("Error: {e}"), win)
    }
}

impl WindowCommand for SetStatus
{
    fn invoke(self)
    {
        self.win
            .imp()
            .status_label
            .set_label(&self.message);
    }
}

impl WindowCommand for Option<SetStatus>
{
    fn invoke(self)
    {
        if let Some(cmd) = self
        {
            cmd.invoke();
        }
    }
}
