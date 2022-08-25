use super::Document;

impl Document
{
    pub fn set_source_path(
        &mut self,
        source_image_path: String,
    )
    {
        self.source_image_path = Some(source_image_path);
    }
}
