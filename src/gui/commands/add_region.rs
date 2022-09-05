use crate::document_window::DocumentWindow;

#[must_use]
pub struct AddRegion<'a>
{
    pub win: &'a DocumentWindow,
}

impl AddRegion<'_>
{
    pub fn invoke(self)
    {
        println!("Add region");
    }
}
