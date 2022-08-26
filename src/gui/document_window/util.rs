use gtk4::traits::WidgetExt;

use super::imp::DocumentWindow;

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

    pub fn refresh_preview(&self)
    {
        self.preview_area.queue_draw();
    }
}
