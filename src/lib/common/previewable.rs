use gtk4::gdk_pixbuf::Pixbuf;

pub trait Previewable
{
    fn create_preview(&self) -> Result<Pixbuf, String>;
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
