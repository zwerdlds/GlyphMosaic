use super::image::DocumentImage;
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    collections::HashSet,
    fs::{
        self,
        File,
    },
    io::Write,
};

#[derive(
    Serialize, Deserialize, Debug, Default, PartialEq,
)]
pub struct Document
{
    pub(crate) source_image: Option<DocumentImage>,
    pub(crate) region_border_pixels: HashSet<(i32, i32)>,
    pub(crate) source_text: Option<String>,
}

impl Document
{
    #[allow(dead_code)]
    pub fn load_from_location(
        path: &str
    ) -> Result<Document, String>
    {
        let data =
            fs::read_to_string(path).map_err(|e| {
                format!("Unable to read file ({e})")
            })?;

        Document::load_from_json(&data)
    }

    #[allow(dead_code)]
    pub fn write_to_location(
        &self,
        path: &str,
    ) -> Result<(), String>
    {
        File::create(path)
            .map_err(|e| {
                format!("Unable to create file ({e})")
            })?
            .write_all(self.serialize_to_json()?.as_bytes())
            .map_err(|e| {
                format!("Unable to write to file ({e})")
            })?;

        Ok(())
    }

    fn load_from_json(
        json: &str
    ) -> Result<Document, String>
    {
        serde_json::from_str(json).map_err(|e| {
            format!("Unable to parse ({e})").into()
        })
    }

    fn serialize_to_json(&self) -> Result<String, String>
    {
        serde_json::to_string_pretty(self).map_err(|e| {
            format!("Unable to serialize document ({e})")
        })
    }
}

#[cfg(test)]
pub mod tests
{
    use super::Document;
    use crate::document::image::tests::generate_test_img;
    use std::collections::HashSet;

    pub fn generate_test_doc() -> Document
    {
        Document {
            source_image: Some(generate_test_img()),
            region_border_pixels: HashSet::new(),
            source_text: Some("Hello world!".to_string()),
        }
    }

    #[test]
    fn simple_document_serialization()
    {
        let doc = generate_test_doc();

        let ser_res_deser = Document::load_from_json(
            &doc.serialize_to_json().unwrap(),
        )
        .unwrap();

        assert_eq!(doc, ser_res_deser);
    }
}
