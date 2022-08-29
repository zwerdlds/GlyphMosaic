use super::imp::DocumentWindow;
use gtk4::traits::WidgetExt;

impl DocumentWindow
{
    pub fn set_status(
        &self,
        message: &str,
    )
    {
        self.status_label.set_label(message);
    }

    pub fn set_status_from_res(
        &self,
        res: Result<String, String>,
    )
    {
        let msg = match res
        {
            Ok(s) => s,
            Err(e) => format!("Error: {e}"),
        };

        self.status_label.set_label(&msg);
    }

    pub fn queue_preview_refresh(&self)
    {
        self.preview_area.queue_draw();
    }

    pub fn apply_command(
        &self,
        cmd: impl glyph_mosaic::commands::DocumentCommand,
    )
    {
        self.model.borrow_mut().apply_command(cmd);
    }
}
