use super::{
    UpdatePreview,
    WindowDocumentCommand,
};
use crate::{
    commands::SetStatus,
    document_window::DocumentWindow,
    model::SettingsTab,
};
use glyph_mosaic::{
    commands::DocumentCommand,
    document::DocumentPoint,
};
use gtk4::subclass::prelude::ObjectSubclassIsExt;

#[must_use]
pub struct PaintCoords<'a>
{
    pub win: &'a DocumentWindow,
    pub pts: Vec<DocumentPoint>,
}

impl PaintCoords<'_>
{
    pub fn invoke(self)
    {
        let res: Result<DocumentCommand, String> = try {
            if self.pts.len() == 0
            {
                Err("Paint points list was empty."
                    .to_string())?
            };

            use SettingsTab::*;

            match self
                .win
                .imp()
                .model
                .borrow()
                .settings_tab()
            {
                Regions =>
                {
                    Ok(DocumentCommand::AddRegionBorder(
                        self.pts.into(),
                    ))
                },
                Sources =>
                {
                    Err("Can't draw in sources or export \
                         mode"
                        .to_string())
                },
                Points =>
                {
                    Err("Can't draw in points mode"
                        .to_string())
                },
                Glyphs =>
                {
                    Err("Can't draw in glyphs mode"
                        .to_string())
                },
                Export =>
                {
                    Err("Can't draw in export mode"
                        .to_string())
                },
                Lines =>
                {
                    Ok(DocumentCommand::AddLineKernel(
                        self.pts.into(),
                    ))
                },
            }?
        };

        match res
        {
            Ok(cmd) =>
            {
                WindowDocumentCommand {
                    command: cmd,
                    win: self.win,
                }
                .invoke()
            },
            Err(msg) =>
            {
                SetStatus {
                    message: format!("Error:{msg}"),
                    win: self.win,
                }
                .invoke()
            },
        };

        UpdatePreview { win: self.win }.invoke();
    }
}
