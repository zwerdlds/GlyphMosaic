use super::PreviewSource;
use crate::{
    commands::PreviewRegions,
    document_window::DocumentWindow,
    model::SettingsTab,
};
use gtk4::{
    self,
    subclass::prelude::ObjectSubclassIsExt,
};

#[must_use]
pub struct UpdatePreview<'a>
{
    pub win: &'a DocumentWindow,
}

impl UpdatePreview<'_>
{
    fn get_tab(&self) -> SettingsTab
    {
        self.win.imp().model.borrow().settings_tab().clone()
    }

    pub fn invoke(self)
    {
        let win = self.win;

        use crate::model::SettingsTab::*;
        match self.get_tab()
        {
            Sources =>
            {
                PreviewSource {
                    win,
                    run_background: true,
                }
                .invoke();
            },
            Regions =>
            {
                PreviewRegions { win }.invoke();
            },
            Lines | Points | Glyphs | Export =>
            {
                println!(
                    "This tab not currently supported."
                )
            },
        }
    }
}
