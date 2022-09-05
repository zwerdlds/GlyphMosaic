use super::Model;
use glyph_mosaic::{
    self,
    document::image::PixbufDef,
};
use gtk4::gdk_pixbuf::Pixbuf;
use std::future::Future;
use tokio::task::JoinHandle;
use uuid::Uuid;

pub type RenderJoinHandle =
    JoinHandle<Result<PixbufDef, String>>;

#[derive(
    Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct RenderHandle(Uuid);

impl Model
{
    pub fn checkout_render<
        F: Future<Output = Result<PixbufDef, String>>,
    >(
        &mut self,
        func: F,
    ) -> RenderHandle
    where
        F: Send + 'static,
    {
        if let Some(r) = &self.current_render
        {
            if !r.is_finished()
            {
                r.abort();
            }
        }

        let new_render = RenderHandle(Uuid::new_v4());
        self.current_render =
            Some(self.tokio_runtime.spawn(func));
        self.current_render_handle = Some(new_render);

        new_render
    }

    pub(crate) fn is_current_render(
        &self,
        current_render_handle: &RenderHandle,
    ) -> bool
    {
        match self.current_render_handle
        {
            None => false,
            Some(self_render) =>
            {
                current_render_handle == &self_render
            },
        }
    }

    pub(crate) fn block_on_current_render(
        &mut self
    ) -> Result<PixbufDef, String>
    {
        match self.current_render.as_mut()
        {
            Some(r) =>
            {
                self.tokio_runtime
                    .block_on(r)
                    .map_err(|e| format!("{e}"))?
            },
            None =>
            {
                Err("No current render to block on."
                    .to_string())
            },
        }
    }

    pub(crate) fn is_current_render_finished(
        &self
    ) -> Option<bool>
    {
        self.current_render
            .as_ref()
            .map(|r| r.is_finished())
    }

    pub(crate) fn current_preview(&self)
        -> &Option<Pixbuf>
    {
        &self.current_preview
    }

    pub(crate) fn set_current_preview(
        &mut self,
        preview: Option<Pixbuf>,
    )
    {
        self.current_preview = preview;
    }
}
