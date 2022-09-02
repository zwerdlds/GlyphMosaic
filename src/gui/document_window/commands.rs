use super::{
    drawing_area_point::DrawingAreaPoint,
    DocumentWindow,
};
use crate::commands::{
    update_drag::DragRelativePoint,
    *,
};
use gtk4::GestureDrag;

impl DocumentWindow
{
    pub fn click(
        &self,
        x: f64,
        y: f64,
    )
    {
        Click::new((x, y).into(), self.clone()).invoke();
    }

    pub fn end_drag(&self)
    {
        EndDrag::new(self.clone()).invoke();
    }

    pub fn prompt_load_source_image(&self)
    {
        PromptLoadSourceImage::new(self.clone()).invoke();
    }

    pub fn prompt_load_source_text(&self)
    {
        PromptLoadSourceText::new(self.clone()).invoke();
    }

    pub fn queue_preview_refresh(&self)
    {
        QueuePreviewRefresh::new(self.clone()).invoke();
    }

    pub fn start_drag(
        &self,
        pt: DrawingAreaPoint,
    )
    {
        StartDrag::new(self.clone(), pt).invoke();
    }

    pub fn update_drag(
        &self,
        dg: GestureDrag,
        rel_pt: DragRelativePoint,
    )
    {
        UpdateDrag::new(self.clone(), dg, rel_pt).invoke();
    }

    pub fn update_settings_tab(
        &self,
        pg: u32,
    )
    {
        UpdateSettingsTab::new(self.clone(), pg).invoke();
    }
}
