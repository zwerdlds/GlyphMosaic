mod image;
mod point;
pub mod render;
mod serialization;
use self::image::DocumentImage;
pub use point::DocumentPoint;
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashSet;

#[derive(
    Serialize, Deserialize, Debug, Default, PartialEq, Clone,
)]
pub struct Document
{
    pub(crate) source_image: Option<DocumentImage>,
    pub(crate) region_border_pixels: HashSet<DocumentPoint>,
    pub(crate) source_text: Option<String>,
}

#[cfg(test)]
pub mod tests
{
    use super::{
        image::tests::generate_test_img,
        Document,
    };
    use std::collections::HashSet;

    pub fn generate_test_doc() -> Document
    {
        Document {
            source_image: Some(generate_test_img()),
            region_border_pixels: HashSet::new(),
            source_text: Some("Hello world!".to_string()),
        }
    }
}
