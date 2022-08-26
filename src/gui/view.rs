use glyph_mosaic::{
    document::properties::DocumentPropertied,
    prelude::*,
};
use gtk4::gdk_pixbuf::Pixbuf;

#[derive(Default)]
pub struct View
{
    document: Document,
    preview_mode: PreviewMode,
}

impl DocumentPropertied for View
{
    fn set_base_image(
        &mut self,
        base_image: Option<Pixbuf>,
    )
    {
        self.document.set_base_image(base_image)
    }
}

#[derive(Default)]
pub enum PreviewMode
{
    #[default]
    BaseImage,
}

impl View
{
    pub fn create_preview(&self) -> Result<Pixbuf, String>
    {
        use PreviewMode::*;

        match self.preview_mode
        {
            BaseImage => self.document.render_base_image(),
        }
    }
}

#[cfg(test)]
mod tests
{
    use gtk4::gdk_pixbuf::{
        Colorspace,
        Pixbuf,
    };

    #[test]
    fn validate_image_crop_immutability()
    {
        let img: Pixbuf = Pixbuf::new(
            Colorspace::Rgb,
            false,
            8,
            512,
            512,
        )
        .unwrap();

        let subimg =
            img.new_subpixbuf(0, 0, 100, 100).unwrap();

        assert_eq!(subimg.width(), 100);
        assert_eq!(subimg.height(), 100);
        assert_eq!(img.height(), 512);
        assert_eq!(img.width(), 512);
    }
}
