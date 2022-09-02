use crate::document_window::DocumentWindow;
use gtk4::subclass::prelude::ObjectSubclassIsExt;

pub struct EndDrag
{
    win: DocumentWindow,
}

impl EndDrag
{
    pub fn new(win: DocumentWindow) -> EndDrag
    {
        EndDrag { win }
    }

    pub fn invoke(self)
    {
        self.win
            .imp()
            .model
            .borrow_mut()
            .set_last_drag_pos(None);
    }
}
