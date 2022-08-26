use crate::document::Document;
use gtk4::gdk_pixbuf::Pixbuf;

impl Document
{
    pub fn render_base_image(
        &self
    ) -> Result<Pixbuf, String>
    {
        self.base_image
            .clone()
            .map(|bi| bi.pixbuf)
            .ok_or("No base image specified.".to_string())
    }
}

#[cfg(test)]
mod tests
{
    use super::Document;
    use gtk4::gdk_pixbuf::Pixbuf;

    #[test]
    fn validate_simple_document_base_image_render()
    {
        let doc = Document {
            base_image: Some(
                Pixbuf::from_file(
                    "./test resources/1x1.png",
                )
                .unwrap()
                .into(),
            ),
        };

        let pixels = doc
            .render_base_image()
            .unwrap()
            .pixel_bytes()
            .unwrap();
        assert_eq!(pixels.len(), 3);
        assert_eq!(pixels[0], 255);
        assert_eq!(pixels[1], 255);
        assert_eq!(pixels[2], 255);
    }
}
