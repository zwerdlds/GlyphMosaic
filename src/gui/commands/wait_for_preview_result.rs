use super::{
    HandlePreviewResult,
    SetStatus,
};
use crate::{
    document_window::DocumentWindow,
    model::RenderHandle,
};
use gtk4::{
    self,
    gdk_pixbuf::Pixbuf,
    glib::{self,},
    prelude::Continue,
    subclass::prelude::ObjectSubclassIsExt,
};
use std::time::Duration;

#[must_use]
pub struct WaitForPreviewResult
{
    pub win: DocumentWindow,
    pub render_handle: RenderHandle,
}

impl WaitForPreviewResult
{
    pub fn invoke(self)
    {
        glib::timeout_add_local(
            Duration::new(0, 1000000),
            move || {
                if self
                    .win
                    .imp()
                    .model
                    .borrow()
                    .is_current_render(&self.render_handle)
                {
                    if Some(true)
                        == self
                            .win
                            .imp()
                            .model
                            .borrow()
                            .is_current_render_finished()
                    {
                        let res: Result<(), String> = try {
                            let img: Pixbuf = self
                                .win
                                .imp()
                                .model
                                .borrow_mut()
                                .block_on_current_render()?
                                .into();

                            HandlePreviewResult {
                                img,
                                win: &self.win,
                            }
                            .invoke();
                        };

                        if let Err(e) = res
                        {
                            SetStatus {
                                message: format!(
                                    "Error from \
                                     background preview \
                                     render: {e}"
                                ),
                                win: &self.win,
                            }
                            .invoke();
                        }

                        Continue(false)
                    }
                    else
                    {
                        Continue(true)
                    }
                }
                else
                {
                    Continue(false)
                }
            },
        );
    }
}
