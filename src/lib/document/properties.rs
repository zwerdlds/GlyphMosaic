use gtk4::gdk_pixbuf::Pixbuf;

use super::{
    doc_img::DocumentImage,
    Document,
};

pub trait DocumentPropertied
{
    fn set_base_image(
        &mut self,
        base_image: Option<Pixbuf>,
    );
}

impl DocumentPropertied for Document
{
    fn set_base_image(
        &mut self,
        base_image: Option<Pixbuf>,
    )
    {
        self.base_image =
            base_image.map(|i| DocumentImage::new(i));
    }
}
