use crate::document::Document;
use image::DynamicImage;

pub trait Previewable
{
    fn create_preview(
        &self,
        document: &Document,
    ) -> Result<DynamicImage, String>;
}

#[cfg(test)]
mod tests
{
    use image::{
        imageops,
        GenericImageView,
        ImageBuffer,
        RgbImage,
    };

    #[test]
    fn validate_image_crop_immutability()
    {
        let mut img: RgbImage = ImageBuffer::new(512, 512);
        let subimg =
            imageops::crop(&mut img, 0, 0, 100, 100);

        assert!(subimg.dimensions() == (100, 100));
        assert!(img.dimensions() == (512, 512));
    }
}
