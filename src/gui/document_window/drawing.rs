use crate::model::SettingsTab;

use super::DocumentWindow;
use glyph_mosaic::{
    commands::{
        AddLineKernel,
        AddRegionBorder,
    },
    document::DocumentPoint,
};
use gtk4::{
    glib::clone,
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
    traits::GestureDragExt,
    GestureDrag,
};

impl DocumentWindow
{
    pub(super) fn setup_drawing(&self)
    {
        let gesture = gtk4::GestureDrag::new();

        gesture.connect_drag_begin(
            clone!(@strong self as win => move |_,x,y|{
                win.drag_start(x,y);
            }),
        );

        gesture.connect_drag_end(
            clone!(@strong self as win => move |_,_,_|{
                win.imp().model.borrow_mut().set_last_drag_pos(None);
            }),
        );

        gesture.connect_drag_update(
            clone!(@strong self as win => move |dg,x,y|{
                win.drag_update(dg,x,y)
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

    fn drag_start(
        &self,
        x: f64,
        y: f64,
    )
    {
        let p = self.preview_area_coord_to_source_coord(
            x as u32, y as u32,
        );
        let res = self.paint_coord(p);
        self.maybe_set_error_from_res(res);
    }

    fn drag_update(
        &self,
        dg: &GestureDrag,
        x: f64,
        y: f64,
    )
    {
        let res = try {
            let from = self
                .imp()
                .model
                .borrow()
                .last_drag_pos()
                .ok_or("Dragged with no start point.")?;
            let sp = dg
                .start_point()
                .ok_or("Drag start point not specified")?;
            let x = (x + sp.0) as u32;
            let y = (y + sp.1) as u32;

            let to = self
                .preview_area_coord_to_source_coord(x, y);

            let pts = from.raster_line_to(to);

            self.paint_coords(pts)?;
        };

        self.maybe_set_error_from_res(res);
    }

    fn click(
        &self,
        x: f64,
        y: f64,
    )
    {
        let p = self.preview_area_coord_to_source_coord(
            x as u32, y as u32,
        );
        let res = self.paint_coord(p);
        self.maybe_set_error_from_res(res);
    }

    fn preview_area_coord_to_source_coord(
        &self,
        x: u32,
        y: u32,
    ) -> DocumentPoint
    {
        let zoom = self.imp().zoom.value();
        DocumentPoint::new(
            ((x as f64) / zoom) as u32,
            ((y as f64) / zoom) as u32,
        )
    }

    fn paint_coord(
        &self,
        pos: DocumentPoint,
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

        let last_pt = pts
            .last()
            .ok_or(
                "Paint point list didn't come with final \
                 point.",
            )?
            .clone();

        use SettingsTab::*;

        match self.imp().model.borrow().settings_tab()
        {
            Regions =>
            {
                let cmd: AddRegionBorder = pts.into();

                self.imp()
                    .model
                    .borrow_mut()
                    .set_last_drag_pos(Some(
                        last_pt.to_owned(),
                    ));

                self.apply_command(cmd);
                self.queue_preview_refresh();

                Ok(())
            },
            Sources =>
            {
                Err("Can't draw in sources or export mode"
                    .to_string())
            },
            Points =>
            {
                Err("Can't draw in points mode".to_string())
            },
            Glyphs =>
            {
                Err("Can't draw in glyphs mode".to_string())
            },
            Export =>
            {
                Err("Can't draw in export mode".to_string())
            },
            Lines =>
            {
                let cmd: AddLineKernel = pts.into();

                self.imp()
                    .model
                    .borrow_mut()
                    .set_last_drag_pos(Some(
                        last_pt.to_owned(),
                    ));

                self.apply_command(cmd);
                self.queue_preview_refresh();

                Ok(())
            },
        }
    }
}
