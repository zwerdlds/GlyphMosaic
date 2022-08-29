use crate::document::Document;
use gtk4::gdk_pixbuf::Pixbuf;

impl Document
{
    pub fn render_source_image(
        &self
    ) -> Result<Pixbuf, String>
    {
        self.source_image
            .clone()
            .map(|bi| bi.pixbuf)
            .ok_or("No base image specified.".to_string())
    }

    pub fn render_regions_image(
        &mut self
    ) -> Result<Pixbuf, String>
    {
        self.source_image
            .clone()
            .map(|bi| bi.pixbuf)
            .ok_or("No region image specified.".to_string())
    }
}

#[cfg(test)]
mod tests
{
    use crate::document::tests::generate_test_doc;

    #[test]
    fn validate_simple_document_source_image_render()
    {
        let doc = generate_test_doc();

        let pixels = doc
            .render_source_image()
            .unwrap()
            .pixel_bytes()
            .unwrap();

        assert_eq!(pixels.len(), 3);
        assert_eq!(pixels[0], 255);
        assert_eq!(pixels[1], 255);
        assert_eq!(pixels[2], 255);
    }
}
