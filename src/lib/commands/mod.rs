// mod set_region_map_image;
// pub use set_region_map_image::*;

mod set_source_image;
pub use set_source_image::*;

mod set_source_text;
pub use set_source_text::*;

mod add_region_border;
pub use add_region_border::*;

use crate::prelude::Document;

pub enum DocumentCommands
{
    DrawRegionBorder(AddRegionBorder),
    SetSourceImage(SetSourceImage),
    SetSourceText(SetSourceText),
}

impl DocumentCommand for DocumentCommands
{
    fn transform_document(
        self,
        doc: &Document,
    ) -> Document
    {
        match self
        {
            DocumentCommands::DrawRegionBorder(c) =>
            {
                c.transform_document(doc)
            },
            DocumentCommands::SetSourceImage(c) =>
            {
                c.transform_document(doc)
            },
            DocumentCommands::SetSourceText(c) =>
            {
                c.transform_document(doc)
            },
        }
    }
}

pub trait DocumentCommand
{
    fn transform_document(
        self,
        doc: &Document,
    ) -> Document;
}
