use super::DocumentWindow;
use crate::commands::redraw_preview::RedrawPreview;
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
        self.setup_load_source_image_button_click();
        self.setup_load_source_text_button_click();
        self.setup_preview_opacity_value_change();
        self.setup_zoom_value_changed();
        self.setup_preview_redraw_request();
        self.setup_drawing();
    }

    fn setup_preview_opacity_value_change(&self)
    {
        self.imp().preview_opacity.connect_value_changed(
            clone!(@strong self as win => move |_| {
               win.queue_preview_refresh();
            }),
        );
    }

    fn setup_zoom_value_changed(&self)
    {
        self.imp().zoom.connect_value_changed(
            clone!(@strong self as win => move |_| {
               win.queue_preview_refresh();
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

    fn setup_drawing(&self)
    {
        self.setup_drag();
        self.setup_click();
    }

    fn setup_click(&self)
    {
        let gesture = gtk4::GestureClick::new();
        gesture.connect_released(
            clone!(@strong self as win => move |_,_,x,y|{
               win.click(x,y);
            }),
        );
        self.imp().preview_area.add_controller(&gesture);
    }

    fn setup_drag(&self)
    {
        let gesture = gtk4::GestureDrag::new();
        gesture.connect_drag_begin(
            clone!(@strong self as win => move |_,x,y|{
               win.start_drag((x,y).into());
            }),
        );
        gesture.connect_drag_end(
            clone!(@strong self as win => move |_,_,_|{
               win.end_drag();
            }),
        );
        gesture.connect_drag_update(
            clone!(@strong self as win => move |dg,x,y|{
               win.update_drag(dg, (x,y).into());
            }),
        );
        self.imp().preview_area.add_controller(&gesture);
    }

    fn setup_load_source_text_button_click(&self)
    {
        self.imp().select_source_text.connect_clicked(
            clone!(@strong self as win => move |_| {
               win.prompt_load_source_text();
            }),
        );
    }

    fn setup_load_source_image_button_click(&self)
    {
        self.imp().select_source_image.connect_clicked(
            clone!(@strong self as win => move |_| {
               win.prompt_load_source_image()
            }),
        );
    }

    fn setup_settings_notebook_tab_switch(&self)
    {
        self.imp().settings_notebook.connect_switch_page(
            clone!(@strong self as win => move |_,_,pg|{
               win.update_settings_tab(pg);
            }),
        );
    }
}
