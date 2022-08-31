use glyph_mosaic::{document::DocumentPoint, commands::AddRegionBorder};
use gtk4::{traits::GestureDragExt, glib::clone};
use super::DocumentWindow;
use gtk4::{
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
};

impl DocumentWindow
{
       pub(super) fn setup_drawing(&self)
    {
        let gesture = gtk4::GestureDrag::new();

        gesture.connect_drag_begin(
            clone!(@strong self as win => move |_,x,y|{
                println!("drag start!");
                let p = win.preview_area_coord_to_source_coord(x as u32,y as u32);
                let res = win.paint_coord(p);
                win.maybe_set_error_from_res(res);
            }),
        );

        gesture.connect_drag_end(
            clone!(@strong self as win => move |_,_,_|{
                println!("drag end!");
                win.imp().model.borrow_mut().set_last_drag_pos(None);
            }),
        );

        gesture.connect_drag_update(
            clone!(@strong self as win => move |dg,x,y|{
                println!("drag update!");
                let res = try {
                    let from = win.imp().model.borrow().last_drag_pos()
                    .ok_or("Dragged with no start point.")?;
                    let sp = dg.start_point().ok_or("Drag start point not specified")?;
                    let x = (x + sp.0) as u32;
                    let y = (y + sp.1) as u32;
                    
                    let to = win.preview_area_coord_to_source_coord(x,y);

                    let pts = from.raster_line_to(
                        to
                    );

                    win.paint_coords(pts)?;
                };

                win.maybe_set_error_from_res(res);
            }),
        );

        self.imp().preview_area.add_controller(&gesture);

        let gesture = gtk4::GestureClick::new();
        gesture.connect_released(
            clone!(@strong self as win => move |_,_,x,y|{
                println!("clicked!");
                let p = win.preview_area_coord_to_source_coord(x as u32,y as u32);
                let res = win.paint_coord(p);
                win.maybe_set_error_from_res(res);
            }),
        );
        self.imp().preview_area.add_controller(&gesture);
    }

    fn preview_area_coord_to_source_coord(
        &self,
        x: u32,
        y: u32,
    ) -> DocumentPoint
    {
        let zoom = self.imp().zoom.value();
        DocumentPoint::new(((x as f64) / zoom) as u32 , ((y as f64)/ zoom) as u32)
    }

    fn paint_coord(
        &self,
        pos: DocumentPoint
    ) -> Result<(), String>
    {
        let cmd: AddRegionBorder = pos.clone().into();

        self.imp()
            .model
            .borrow_mut()
            .set_last_drag_pos(Some(pos));

        self.apply_command(cmd);
        self.queue_preview_refresh();

        Ok(())
    }

    fn paint_coords(
        &self,
        pts: Vec<DocumentPoint>,
    ) -> Result<(), String>
    {
        if pts.len() == 0
        {
            return Err(
                "Paint points list was empty.".to_string()
            );
        };

        let last_pt = pts.last().ok_or(
            "Paint point list didn't come with final \
             point.",
        )?.clone();

        let cmd: AddRegionBorder = pts.into();

        self.imp()
            .model
            .borrow_mut()
            .set_last_drag_pos(Some(last_pt.to_owned()));

        self.apply_command(cmd);
        self.queue_preview_refresh();

        Ok(())
    }
}