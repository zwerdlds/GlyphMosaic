use super::DocumentTransformable;
use crate::prelude::Document;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct SetSourceText
{
    source_text: Option<String>,
}

impl SetSourceText
{
    pub fn new(source_text: Option<String>) -> Self
    {
        SetSourceText { source_text }
    }
}

impl From<Option<String>> for SetSourceText
{
    fn from(text: Option<String>) -> Self
    {
        SetSourceText::new(text)
    }
}

impl From<String> for SetSourceText
{
    fn from(text: String) -> Self
    {
        Some(text).into()
    }
}

impl DocumentTransformable for SetSourceText
{
    fn transform_document(
        self,
        doc: &Document,
    ) -> Document
    {
        let mut doc = doc.clone();

        doc.source_text = self.source_text;

        doc
    }
}

#[cfg(test)]
pub mod tests
{
    use crate::{
        commands::{
            DocumentTransformable,
            SetSourceText,
        },
        document::tests::generate_test_doc,
    };

    #[test]
    fn set_text()
    {
        let doc = generate_test_doc();

        assert_eq!(
            doc.clone().source_text.unwrap(),
            "Hello world!"
        );

        let cmd = SetSourceText {
            source_text: Some("Goodbye world!".to_string()),
        };

        let res_doc = cmd.transform_document(&doc);

        assert_eq!(
            res_doc.source_text.unwrap(),
            "Goodbye world!"
        );
    }
}
