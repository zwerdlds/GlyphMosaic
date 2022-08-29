use delegate::delegate;
use glyph_mosaic::{
    document::properties::DocumentPropertied,
    prelude::*,
};
use gtk4::{
    gdk_pixbuf::Pixbuf,
    Adjustment,
};

pub struct Model
{
    document: Document,
    preview_mode: PreviewMode,
    pub zoom_adj: Adjustment,
}

impl Default for Model
{
    fn default() -> Self
    {
        let document = Document::default();
        let preview_mode = PreviewMode::BaseImage;
        let zoom_adj = Adjustment::new(
            1f64, 0f64, 30f64, 0.1f64, 1f64, 1f64,
        );

        Model {
            document,
            preview_mode,
            zoom_adj,
        }
    }
}

impl DocumentPropertied for Model
{
    delegate! {
        to self.document {
            fn set_source_image(&mut self, image: Option<Pixbuf>);
            fn set_source_text(&mut self, text: Option<String>);
        }
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
            BaseImage =>
            {
                self.document.render_source_image()
            },
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
