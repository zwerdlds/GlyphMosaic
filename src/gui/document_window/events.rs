use super::DocumentWindow;
use crate::commands::{
    RedrawPreview,
    *,
};
use gtk4::{
    self,
    glib::clone,
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
};

impl DocumentWindow
{
    pub fn setup_events(&self)
    {
        self.setup_settings_notebook_tab_switch();
        self.setup_load_regions_map_image_button_click();
        self.setup_load_source_image_button_click();
        self.setup_load_source_text_button_click();
        self.setup_regions_buttons_click();
        self.setup_preview_opacity_value_change();
        self.setup_zoom_value_changed();
        self.setup_preview_redraw_request();
        self.setup_mouse();
    }

    fn setup_regions_buttons_click(&self)
    {
        self.imp().add_region.connect_clicked(
            clone!(@strong self as win => move |_| {
                AddRegion{win:&win}.invoke();
            }),
        );
        self.imp().remove_region.connect_clicked(
            clone!(@strong self as win => move |_| {
                RemoveRegion{win:&win}.invoke();
            }),
        );
    }

    fn setup_preview_opacity_value_change(&self)
    {
        self.imp().preview_opacity.connect_value_changed(
            clone!(@strong self as win => move |_| {
                UpdatePreview { win:&win }.invoke();
            }),
        );
    }

    fn setup_zoom_value_changed(&self)
    {
        self.imp().zoom.connect_value_changed(
            clone!(@strong self as win => move |_| {
                UpdatePreview { win:&win }.invoke();
            }),
        );
    }

    fn setup_preview_redraw_request(&self)
    {
        self.imp().preview_area.set_draw_func(
            clone!(@strong self as win => move |area, ctx, _width, _height| {
                    RedrawPreview{area: area,
                        win:&win,ctx}.invoke();
            })
        );
    }

    fn setup_mouse(&self)
    {
        self.setup_drag();
        self.setup_click();
    }

    fn setup_click(&self)
    {
        let gesture = gtk4::GestureClick::new();
        gesture.connect_released(
            clone!(@strong self as win => move |_,_,x,y|{
                Click{win:&win,pt:(x,y).into()}.invoke();
            }),
        );
        self.imp().preview_area.add_controller(&gesture);
    }

    fn setup_drag(&self)
    {
        let gesture = gtk4::GestureDrag::new();
        gesture.connect_drag_begin(
            clone!(@strong self as win => move |_,x,y|{
                StartDrag{win:&win,pt:(x,y).into()}.invoke();
            }),
        );
        gesture.connect_drag_end(
            clone!(@strong self as win => move |_,_,_|{
                EndDrag{win:&win}.invoke();
            }),
        );
        gesture.connect_drag_update(
            clone!(@strong self as win => move |dg,x,y|{
                UpdateDrag{win:&win,dg,pt:(x,y).into()}.invoke();
            }),
        );
        self.imp().preview_area.add_controller(&gesture);
    }

    fn setup_load_source_text_button_click(&self)
    {
        self.setup_regions_buttons_click();
    }

    fn setup_load_source_image_button_click(&self)
    {
        self.imp().select_source_image.connect_clicked(
            clone!(@strong self as win => move |_| {
                PromptLoadSourceImage{win:&win}.invoke();
            }),
        );
    }

    fn setup_load_regions_map_image_button_click(&self)
    {
        self.imp().select_regions_map_image.connect_clicked(
            clone!(@strong self as win => move |_| {
                PromptLoadRegionsMapImage{win:&win}.invoke();
            }),
        );
    }

    fn setup_settings_notebook_tab_switch(&self)
    {
        self.imp().settings_notebook.connect_switch_page(
            clone!(@strong self as win => move |_,_,page_index|{
                UpdateSettingsTab{win:&win, page_index}.invoke();
            }),
        );
    }
}
