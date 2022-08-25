use crate::{
    document::{
        Document,
        PreviewMode,
    },
    prelude::*,
};
use gtk4::gdk_pixbuf::Pixbuf;

impl Previewable for Document
{
    fn create_preview(&self) -> Result<Pixbuf, String>
    {
        use PreviewMode::*;

        match self.preview_mode
        {
            Source => self.create_source_image_preview(),
        }
    }
}

impl Document
{
    fn create_source_image_preview(
        &self
    ) -> Result<Pixbuf, String>
    {
        Pixbuf::from_file(
            self.source_image_path
                .as_ref()
                .ok_or("No base image to preview")?
                .clone(),
        )
        .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests
{
    use super::Document;
    use crate::prelude::*;

    #[test]
    fn validate_simple_document_source_preview()
    {
        let doc = Document {
            preview_mode: PreviewMode::Source,
            source_image_path: Some(
                "./test resources/1x1.png".to_string(),
            ),
        };

        let pixels = doc
            .create_preview()
            .unwrap()
            .pixel_bytes()
            .unwrap();
        assert_eq!(pixels.len(), 3);
        assert_eq!(pixels[0], 255);
        assert_eq!(pixels[1], 255);
        assert_eq!(pixels[2], 255);
    }
}
