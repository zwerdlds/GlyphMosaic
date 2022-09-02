pub(crate) mod click;
pub(crate) mod end_drag;
pub(crate) mod handle_load_source_image_response;
pub(crate) mod handle_load_source_text_response;
pub(crate) mod paint_coords;
pub(crate) mod prompt_load_source_image;
pub(crate) mod prompt_load_source_text;
pub(crate) mod queue_preview_refresh;
pub(crate) mod redraw_preview;
pub(crate) mod set_status;
pub(crate) mod start_drag;
pub(crate) mod update_drag;
pub(crate) mod update_settings_tab;
pub(crate) mod window_document_command;

pub use click::Click;
pub use end_drag::EndDrag;
pub use handle_load_source_image_response::HandleLoadSourceImageResponse;
pub use handle_load_source_text_response::HandleLoadSourceTextResponse;
pub use paint_coords::PaintCoords;
pub use prompt_load_source_image::PromptLoadSourceImage;
pub use prompt_load_source_text::PromptLoadSourceText;
pub use queue_preview_refresh::QueuePreviewRefresh;
pub use redraw_preview::RedrawPreview;
pub use set_status::SetStatus;
pub use start_drag::StartDrag;
pub use update_drag::UpdateDrag;
pub use update_settings_tab::UpdateSettingsTab;
pub use window_document_command::WindowDocumentCommand;

pub trait WindowCommand
{
    fn invoke(self);
}
