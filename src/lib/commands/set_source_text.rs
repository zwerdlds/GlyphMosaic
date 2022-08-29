use super::DocumentCommand;
use crate::prelude::Document;

pub struct SetSourceText(pub Option<String>);

impl DocumentCommand for SetSourceText
{
    fn transform_document(
        self,
        doc: &Document,
    ) -> Document
    {
        Document {
            source_image: doc.source_image.clone(),
            region_border_pixels: doc
                .region_border_pixels
                .clone(),
            source_text: self.0,
        }
    }
}

#[cfg(test)]
pub mod tests
{
    use crate::{
        commands::{
            DocumentCommandApplyable,
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

        let cmd = SetSourceText(Some(
            "Goodbye world!".to_string(),
        ));

        let res_doc = doc.apply_command(cmd);

        assert_eq!(
            res_doc.source_text.unwrap(),
            "Goodbye world!"
        );
        todo!("Add assertions for dependent values.");
    }
}
