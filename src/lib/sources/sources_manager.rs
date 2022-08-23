use crate::{
    common::Previewable,
    document::Document,
};
use image::{
    io::Reader as ImageReader,
    DynamicImage,
};

pub struct SourcesManager {}

impl SourcesManager
{
}

impl Previewable for SourcesManager
{
    fn create_preview(
        &self,
        document: &Document,
    ) -> Result<DynamicImage, String>
    {
        ImageReader::open(
            document.source_image_path.clone(),
        )
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests
{
    use super::{
        Document,
        SourcesManager,
    };
    use crate::{
        common::Previewable,
        document::PreviewMode,
    };
    use image::RgbImage;

    #[test]
    fn validate_simple_document_source_preview()
    {
        let sm = SourcesManager {};

        let doc = Document {
            preview_mode: PreviewMode::Source,
            source_image_path: "./test resources/1x1.png"
                .to_string(),
        };

        let preview =
            sm.create_preview(&doc).unwrap().to_rgb8();

        let mut constructed = RgbImage::new(1, 1);
        constructed.get_pixel_mut(0, 0).0 = [255, 255, 255];

        assert_eq!(preview, constructed);
    }
}
