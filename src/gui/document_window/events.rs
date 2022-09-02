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
        self.imp().settings_notebook.connect_switch_page(
            clone!(@strong self as win => move |_,_,pg|{
               win.update_settings_tab(pg);
            }),
        );

        self.imp().select_source_image.connect_clicked(
            clone!(@strong self as win => move |_| {
               win.prompt_load_source_image()
            }),
        );

        self.imp().select_source_text.connect_clicked(
            clone!(@strong self as win => move |_| {
               win.prompt_load_source_text();
            }),
        );

        self.imp().preview_opacity.connect_value_changed(
            clone!(@strong self as win => move |_| {
               win.queue_preview_refresh();
            }),
        );

        self.imp().zoom.connect_value_changed(
            clone!(@strong self as win => move |_| {
               win.queue_preview_refresh();
            }),
        );

        self.imp().preview_area.set_draw_func(
            clone!(@strong self as win => move |area, ctx, _width, _height| {
                RedrawPreview::new(area.to_owned(),win.to_owned(),ctx.to_owned()).invoke();
            })
        );

        // Drawing
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
               win.update_drag(dg.clone(), (x,y).into());
            }),
        );

        self.imp().preview_area.add_controller(&gesture);

        let gesture = gtk4::GestureClick::new();
        gesture.connect_released(
            clone!(@strong self as win => move |_,_,x,y|{
               win.click(x,y);
            }),
        );
        self.imp().preview_area.add_controller(&gesture);
    }
}
