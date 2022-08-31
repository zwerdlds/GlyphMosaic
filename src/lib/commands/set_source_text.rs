use super::DocumentCommand;
use crate::prelude::Document;

pub struct SetSourceText
{
    pub source_text: Option<String>,
}

impl DocumentCommand for SetSourceText
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
            DocumentCommand,
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
