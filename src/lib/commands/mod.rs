// mod set_region_map_image;
// pub use set_region_map_image::*;

mod set_source_image;
pub use set_source_image::*;

mod set_source_text;
pub use set_source_text::*;

mod add_region_border;
pub use add_region_border::*;

mod add_line_kernel;
pub use add_line_kernel::*;

use crate::prelude::Document;

pub enum DocumentCommands
{
    AddLineKernel(AddLineKernel),
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
        use DocumentCommands::*;

        match self
        {
            DrawRegionBorder(c) => td(c, doc),
            SetSourceImage(c) => td(c, doc),
            SetSourceText(c) => td(c, doc),
            AddLineKernel(c) => td(c, doc),
        }
    }
}

fn td(
    c: impl DocumentCommand,
    doc: &Document,
) -> Document
{
    c.transform_document(doc)
}

pub trait DocumentCommand
{
    fn transform_document(
        self,
        doc: &Document,
    ) -> Document;
}
