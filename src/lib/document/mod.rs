mod image;
pub mod render;
mod serialization;

use crate::commands::DocumentCommandApplyable;

use self::image::DocumentImage;
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
    pub(crate) region_border_pixels: HashSet<(i32, i32)>,
    pub(crate) source_text: Option<String>,
}

impl DocumentCommandApplyable for Document
{
    fn apply_command(
        &self,
        cmd: impl crate::commands::DocumentCommand,
    ) -> Document
    {
        cmd.transform_document(self)
    }
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
