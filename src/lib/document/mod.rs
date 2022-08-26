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
    base_image: Option<DocumentImage>,
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
mod tests
{
    use gtk4::gdk_pixbuf::Pixbuf;

    use super::Document;
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
            base_image: Some(
                Pixbuf::from_file(
                    "./test resources/1x1.png",
                )
                .unwrap()
                .into(),
            ),
        };

        let serialize_res =
            Document::serialize_to_json(&doc).unwrap();

        print!("{:?}", serialize_res);

        assert_eq!(
            serialize_res,
            "{\n  \"base_image\": {\n    \"pixbuf\": {\n      \"data\": [\n        255,\n        255,\n        255\n      ],\n      \"has_alpha\": false,\n      \"bits_per_sample\": 8,\n      \"width\": 1,\n      \"height\": 1,\n      \"rowstride\": 4\n    }\n  }\n}"
        );
    }

    #[test]
    fn validate_simple_document_load_json()
    {
        let doc = include_str!(
            "../../../test resources/testdoc.json"
        );

        let load_res =
            Document::load_from_json(doc).unwrap();

        let gen_doc = Document {
            base_image: Some(
                Pixbuf::from_file(
                    "./test resources/1x1.png",
                )
                .unwrap()
                .into(),
            ),
        };

        assert_eq!(load_res.base_image, gen_doc.base_image);
    }

    #[test]
    fn validate_simple_document_load_from_path()
    {
        let doc = Document::load_from_location(
            "./test resources/testdoc.json",
        )
        .unwrap();

        assert_eq!(
            doc,
            Document {
                base_image: Some(
                    Pixbuf::from_file(
                        "./test resources/1x1.png"
                    )
                    .unwrap()
                    .into()
                )
            }
        );
    }
}
