use super::DocumentTransformable;
use crate::prelude::{
    Document,
    DocumentImage,
};
use gtk4::gdk_pixbuf::Pixbuf;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct SetSourceImage
{
    source_image: Option<DocumentImage>,
}

impl SetSourceImage
{
    pub fn new(
        source_image: Option<DocumentImage>
    ) -> SetSourceImage
    {
        SetSourceImage { source_image }
    }
}

impl DocumentTransformable for SetSourceImage
{
    fn transform_document(
        self,
        doc: &Document,
    ) -> Document
    {
        let mut doc = doc.clone();

        doc.source_image = self.source_image;

        doc
    }
}

impl From<Option<DocumentImage>> for SetSourceImage
{
    fn from(i: Option<DocumentImage>) -> Self
    {
        SetSourceImage::new(i)
    }
}

impl From<DocumentImage> for SetSourceImage
{
    fn from(i: DocumentImage) -> Self
    {
        Some(i).into()
    }
}

impl From<Pixbuf> for SetSourceImage
{
    fn from(i: Pixbuf) -> Self
    {
        Some(i.into()).into()
    }
}
