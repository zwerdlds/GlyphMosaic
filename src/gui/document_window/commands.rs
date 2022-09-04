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
        Click {
            pt: (x, y).into(),
            win: self,
        }
        .invoke();
    }

    pub fn end_drag(&self)
    {
        EndDrag { win: self }.invoke();
    }

    pub fn prompt_load_source_image(&self)
    {
        PromptLoadSourceImage { win: self }.invoke();
    }

    pub fn prompt_load_source_text(&self)
    {
        PromptLoadSourceText { win: self }.invoke();
    }

    pub fn queue_preview_refresh(&self)
    {
        QueuePreviewRefresh { win: self }.invoke();
    }

    pub fn start_drag(
        &self,
        pt: DrawingAreaPoint,
    )
    {
        StartDrag { win: self, pt }.invoke();
    }

    pub fn update_drag(
        &self,
        dg: &GestureDrag,
        rel_pt: DragRelativePoint,
    )
    {
        UpdateDrag {
            win: self,
            dg,
            pt: rel_pt,
        }
        .invoke();
    }

    pub fn update_settings_tab(
        &self,
        pg: u32,
    )
    {
        UpdateSettingsTab {
            win: self,
            page_index: pg,
        }
        .invoke();
    }
}
