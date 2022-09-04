use super::PreviewSource;
use crate::{
    commands::PreviewRegions,
    document_window::DocumentWindow,
    model::SettingsTab,
};
use gtk4::{
    self,
    cairo::Context,
    subclass::prelude::ObjectSubclassIsExt,
    DrawingArea,
};

#[must_use]
pub struct RedrawPreview<'a>
{
    pub area: &'a DrawingArea,
    pub win: &'a DocumentWindow,
    pub ctx: &'a Context,
}

impl RedrawPreview<'_>
{
    fn get_tab(&self) -> SettingsTab
    {
        self.win.imp().model.borrow().settings_tab().clone()
    }

    pub fn invoke(self)
    {
        let area = self.area;
        let win = self.win;
        let ctx = self.ctx;

        use crate::model::SettingsTab::*;
        match self.get_tab()
        {
            Sources =>
            {
                PreviewSource { area, win, ctx }.invoke();
            },
            Regions =>
            {
                PreviewRegions { area, win, ctx }.invoke();
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
