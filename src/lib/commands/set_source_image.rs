use super::DocumentCommand;
use crate::prelude::Document;
use gtk4::gdk_pixbuf::Pixbuf;

pub struct SetSourceImage(pub Option<Pixbuf>);

impl DocumentCommand for SetSourceImage
{
    fn transform_document(
        self,
        doc: &Document,
    ) -> Document
    {
        let mut doc = doc.clone();

        let source_image = self.0.map(|i| i.into());

        doc.source_image = source_image;

        doc
    }
}
