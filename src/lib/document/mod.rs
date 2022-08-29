mod doc_img;
pub mod properties;
pub mod render;
use serde::{
    Deserialize,
    Serialize,
};
use std::fs;

use self::doc_img::DocumentImage;

#[derive(
    Serialize, Deserialize, Debug, Default, PartialEq,
)]
pub struct Document
{
    source_image: Option<DocumentImage>,
    source_text: Option<String>,
}

impl Document
{
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

    pub fn load_from_json(
        json: &str
    ) -> Result<Document, String>
    {
        serde_json::from_str(json).map_err(|e| {
            format!("Unable to parse ({e})").into()
        })
    }

    pub fn serialize_to_json(
        document: &Document
    ) -> Result<String, String>
    {
        serde_json::to_string_pretty(document).map_err(
            |e| {
                format!(
                    "Unable to serialize document ({e})"
                )
            },
        )
    }
}

#[cfg(test)]
pub mod tests
{
    use super::{
        doc_img::tests::generate_test_img,
        Document,
    };

    pub fn generate_test_doc() -> Document
    {
        Document {
            source_image: Some(generate_test_img()),
            source_text: Some("Hello world!".to_string()),
        }
    }

    #[test]
    fn validate_simple_document_serialization()
    {
        let doc = generate_test_doc();

        let ser_res_deser = Document::load_from_json(
            &Document::serialize_to_json(&doc).unwrap(),
        )
        .unwrap();

        assert_eq!(doc, ser_res_deser);
    }
}
