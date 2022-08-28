use glyph_mosaic::{
    document::properties::DocumentPropertied,
    prelude::*,
};
use gtk4::{
    gdk_pixbuf::Pixbuf,
    graphene::Point,
};

pub struct Model
{
    document: Document,
    preview_mode: PreviewMode,
    pub image_center_offset: Point,
}

impl Default for Model
{
    fn default() -> Self
    {
        Model {
            document: Document::default(),
            preview_mode: PreviewMode::BaseImage,
            image_center_offset: Point::new(0f32, 0f32),
        }
    }
}

impl DocumentPropertied for Model
{
    fn set_base_image(
        &mut self,
        base_image: Option<Pixbuf>,
    )
    {
        self.document.set_base_image(base_image)
    }
}

pub enum PreviewMode
{
    BaseImage,
}

impl Model
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
