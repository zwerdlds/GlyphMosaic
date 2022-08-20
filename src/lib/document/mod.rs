use serde::{
    Deserialize,
    Serialize,
};
use std::fs;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Document
{
    pub preview_mode: PreviewMode,
    pub source_image_path: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum PreviewMode
{
    Source,
}

impl Document
{
    pub fn load_from_location(
        path: &str
    ) -> Result<Document, String>
    {
        let data = fs::read_to_string(path)
            .map_err(|_| "Unable to read file")?;

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
                    "Unable to serialize document ({:?})",
                    e
                )
                .into()
            },
        )
    }
}

#[cfg(test)]
mod tests
{
    use super::Document;
    use crate::document::PreviewMode;
    use std::path::{
        Path,
        PathBuf,
    };

    #[test]
    fn validate_path_canonicalization()
    {
        let path = Path::new("./test resources/1x1.png");
        let canon = path.canonicalize().unwrap();

        assert_eq!(
            canon,
            PathBuf::from(
                "/mnt/Speedy/Development/GlyphMosaic/test \
                 resources/1x1.png"
            )
        );
    }

    #[test]
    fn validate_simple_document_serialization()
    {
        let doc = Document {
            preview_mode: PreviewMode::Source,
            source_image_path: "./test resources/1x1.png"
                .to_string(),
        };

        let serialize_res =
            Document::serialize_to_json(&doc);

        // print!("{:?}", serialize_res);

        assert_eq!(
            serialize_res,
            Ok("{\n  \"preview_mode\": \"Source\",\n  \
                \"source_image_path\": \"./test \
                resources/1x1.png\"\n}"
                .to_string())
        );
    }

    #[test]
    fn validate_simple_document_load_json()
    {
        let doc = include_str!(
            "../../../test resources/testdoc.json"
        );

        let load_res = Document::load_from_json(doc);
        assert_eq!(
            load_res,
            Ok(Document {
                preview_mode: PreviewMode::Source,
                source_image_path: "./test resources/1x1.\
                                    png"
                .to_string()
            })
        );
    }

    #[test]
    fn validate_simple_document_load_from_path()
    {
        let doc = Document::load_from_location(
            "./test resources/testdoc.json",
        );

        assert_eq!(
            doc,
            Ok(Document {
                preview_mode: PreviewMode::Source,
                source_image_path: "./test resources/1x1.\
                                    png"
                .to_string()
            })
        );
    }
}
