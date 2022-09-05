use super::{
    SetStatus,
    UpdatePreview,
};
use crate::document_window::DocumentWindow;
use glyph_mosaic::document::Document;
use gtk4::subclass::prelude::ObjectSubclassIsExt;

#[must_use]
pub struct LoadDocument<'a>
{
    pub win: &'a DocumentWindow,
    pub load_from_path: String,
}

impl LoadDocument<'_>
{
    pub fn invoke(self)
    {
        let result: Result<String, String> = try {
            let path = self.load_from_path;

            let doc = Document::load_from_location(&path)?;

            self.win
                .imp()
                .model
                .borrow_mut()
                .set_document(doc);

            self.win
                .imp()
                .model
                .borrow_mut()
                .set_document_path(Some(path.clone()));

            format!("Loaded document from {path}")
                .to_string()
        };

        let message = match result
        {
            Ok(m) => m,
            Err(e) =>
            {
                format!("Error loading document: {e}")
            },
        };
        SetStatus {
            message,
            win: self.win,
        }
        .invoke();
        UpdatePreview { win: self.win }.invoke();
    }
}
