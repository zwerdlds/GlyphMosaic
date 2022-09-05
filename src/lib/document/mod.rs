mod document_point;
pub mod image;
pub mod region;
pub mod serialization;
use self::{
    image::DocumentImage,
    region::DocumentRegion,
};
pub use document_point::DocumentPoint;
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::{
    HashMap,
    HashSet,
};

#[derive(
    Serialize, Deserialize, Debug, PartialEq, Default, Clone,
)]
pub struct Document
{
    pub(crate) source_image: Option<DocumentImage>,
    pub(crate) region_border_pixels: HashSet<DocumentPoint>,
    pub(crate) regions:
        HashMap<DocumentPoint, DocumentRegion>,
    pub(crate) source_text: Option<String>,
}

impl Document
{
    pub fn region_border_pixels(
        &self
    ) -> &HashSet<DocumentPoint>
    {
        &self.region_border_pixels
    }

    pub fn source_image(&self) -> &Option<DocumentImage>
    {
        &self.source_image
    }
}

#[cfg(test)]
pub mod tests
{
    use super::{
        image::tests::generate_test_img,
        Document,
    };
    use std::collections::{
        HashMap,
        HashSet,
    };

    pub fn generate_test_doc() -> Document
    {
        let source_image = Some(generate_test_img());
        let region_border_pixels = HashSet::new();
        let source_text = Some("Hello world!".to_string());
        let regions = HashMap::new();

        Document {
            regions,
            source_image,
            region_border_pixels,
            source_text,
        }
    }
}
