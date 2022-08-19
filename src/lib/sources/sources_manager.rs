use crate::{
    cache::cache::Cache,
    common::Previewable,
    document::Document,
};
use image::{
    io::Reader as ImageReader,
    DynamicImage,
};
use std::rc::Rc;

pub struct SourcesManager
{
    _cache: Rc<Box<Cache>>,
}

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
