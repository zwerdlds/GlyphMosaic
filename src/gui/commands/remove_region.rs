use crate::document_window::DocumentWindow;

#[must_use]
pub struct RemoveRegion<'a>
{
    pub win: &'a DocumentWindow,
}

impl RemoveRegion<'_>
{
    pub fn invoke(self)
    {
        println!("Remove region");
    }
}
