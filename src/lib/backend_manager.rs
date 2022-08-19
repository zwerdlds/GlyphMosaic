use crate::{
    common::Previewable,
    document::{
        Document,
        PreviewMode,
    },
    sources::SourcesManager,
};
use image::DynamicImage;

pub struct BackendManager
{
    sources_manager: SourcesManager,
}

impl Previewable for BackendManager
{
    fn create_preview(
        &self,
        document: &Document,
    ) -> Result<DynamicImage, String>
    {
        use PreviewMode::*;

        match &document.preview_mode
        {
            Source => &self.sources_manager,
        }
        .create_preview(document)
    }
}
