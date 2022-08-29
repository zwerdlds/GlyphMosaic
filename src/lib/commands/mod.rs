// mod set_region_map_image;
// pub use set_region_map_image::*;

mod set_source_image;
pub use set_source_image::*;

mod set_source_text;
pub use set_source_text::*;

use crate::prelude::Document;

pub trait DocumentCommand
{
    fn transform_document(
        self,
        doc: &Document,
    ) -> Document;
}

pub trait DocumentCommandApplyable
{
    fn apply_command(
        &self,
        cmd: impl DocumentCommand,
    ) -> Document;
}
