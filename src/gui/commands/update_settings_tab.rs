use gtk4::subclass::prelude::ObjectSubclassIsExt;

use super::WindowCommand;
use crate::{
    commands::{
        QueuePreviewRefresh,
        SetStatus,
    },
    document_window::DocumentWindow,
    model::SettingsTab,
};

pub struct UpdateSettingsTab
{
    win: DocumentWindow,
    page_index: u32,
}

impl UpdateSettingsTab
{
    pub fn new(
        win: DocumentWindow,
        page_index: u32,
    ) -> UpdateSettingsTab
    {
        UpdateSettingsTab { win, page_index }
    }
}

impl WindowCommand for UpdateSettingsTab
{
    fn invoke(self)
    {
        use SettingsTab::*;

        let p: Result<SettingsTab, String> = match self
            .page_index
        {
            0 => Ok(Sources),
            1 => Ok(Regions),
            2 => Ok(Lines),
            3 => Ok(Points),
            4 => Ok(Glyphs),
            5 => Ok(Export),
            i =>
            {
                Err(format!(
                    "Tab index {i} is not supported."
                ))
            },
        };

        match p
        {
            Ok(p) =>
            {
                self.win
                    .imp()
                    .model
                    .borrow_mut()
                    .set_settings_tab(p);

                QueuePreviewRefresh::new(self.win).invoke();
            },
            Err(e) =>
            {
                SetStatus::new_error(
                    format!("Switching tab: {e}"),
                    self.win,
                )
                .invoke()
            },
        }
    }
}
