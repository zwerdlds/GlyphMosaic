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
        match res
        {
            Ok(msg) => self.set_status(&msg),
            Err(e) => self.set_error(e),
        };
    }

    pub fn maybe_set_error_from_res(
        &self,
        res: Result<(), String>,
    )
    {
        if let Err(msg) = res
        {
            self.set_error(msg);
        };
    }

    fn set_error(
        &self,
        e: String,
    )
    {
        self.set_status(format!("Error: {e}").as_str());
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
