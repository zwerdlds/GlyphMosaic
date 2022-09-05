use crate::{
    commands::{
        SetStatus,
        UpdatePreview,
    },
    document_window::DocumentWindow,
    model::SettingsTab,
};
use gtk4::subclass::prelude::ObjectSubclassIsExt;

#[must_use]
pub struct UpdateSettingsTab<'a>
{
    pub win: &'a DocumentWindow,
    pub page_index: u32,
}

impl UpdateSettingsTab<'_>
{
    pub fn invoke(self)
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

                UpdatePreview { win: self.win }.invoke();
            },
            Err(e) =>
            {
                SetStatus {
                    message: format!("Switching tab: {e}")
                        .to_string(),
                    win: self.win,
                }
                .invoke()
            },
        }
    }
}
