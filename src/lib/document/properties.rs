use gtk4::gdk_pixbuf::Pixbuf;

use super::Document;

pub trait DocumentPropertied
{
    fn set_source_image(
        &mut self,
        image: Option<Pixbuf>,
    );

    fn set_source_text(
        &mut self,
        text: Option<String>,
    );
}

impl DocumentPropertied for Document
{
    fn set_source_image(
        &mut self,
        image: Option<Pixbuf>,
    )
    {
        self.source_image = image.map(|i| i.into());
    }

    fn set_source_text(
        &mut self,
        txt: Option<String>,
    )
    {
        self.source_text = txt;
    }
}
