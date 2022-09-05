use super::{
    SetStatus,
    UpdatePreview,
};
use crate::{
    commands::WindowDocumentCommand,
    document_window::DocumentWindow,
    util::get_dialog_path,
};
use glyph_mosaic::{
    commands::{
        AddRegionBorder,
        DocumentCommand,
    },
    document::DocumentPoint,
};
use gtk4::{
    gdk_pixbuf::Pixbuf,
    glib::clone,
    traits::{
        DialogExt,
        GtkWindowExt,
        WidgetExt,
    },
    FileChooserAction,
    FileChooserDialog,
    ResponseType,
};
use std::collections::HashSet;

#[must_use]
pub struct PromptLoadRegionsMapImage<'a>
{
    pub win: &'a DocumentWindow,
}

impl PromptLoadRegionsMapImage<'_>
{
    pub fn invoke(self)
    {
        let load_regions_map_dialog =
            FileChooserDialog::new(
                Some("Select Regions Map Image"),
                Some(self.win),
                FileChooserAction::Open,
                &[
                    ("Open", ResponseType::Ok),
                    ("Cancel", ResponseType::Cancel),
                ],
            );

        load_regions_map_dialog.connect_response(
            clone!(@strong self.win as win =>
            move |dialog: &FileChooserDialog,
              response: ResponseType| {
                handle_load_regions_map_dialog_response(&win,dialog,response);
              }),
        );

        load_regions_map_dialog.show();
    }
}

fn handle_load_regions_map_dialog_response(
    win: &DocumentWindow,
    dialog: &FileChooserDialog,
    response: ResponseType,
)
{
    dialog.close();

    if response != ResponseType::Ok
    {
        return;
    }

    let result: Result<String, String> = try {
        let file_path = get_dialog_path(dialog)?;

        let img = Pixbuf::from_file(file_path.clone())
            .map_err(|e| {
                format!(
                    "Failed getting Region Map image \
                     data: {e}"
                )
            })?;

        let w = img.width() as usize;
        let h = img.height() as usize;
        if !img.has_alpha()
        {
            Err("Region Map Image does not have alpha \
                 channel.")?;
        }

        let n_channels = img.n_channels();
        let rowstride = img.rowstride();
        let bytes = img
            .read_pixel_bytes()
            .ok_or("Image pixel byte read failed.")?;

        let px_at = |(x, y)| {
            let byte_addr = ((y as i32 * rowstride)
                + (x as i32 * n_channels)
                + 3) as usize;
            bytes[byte_addr]
        };

        let region_borders: HashSet<DocumentPoint> = (0..w)
            .map(|x| (0..h).map(move |y| (x, y)))
            .flatten()
            .filter(|pt| 0 != px_at(*pt))
            .map(From::from)
            .collect();

        WindowDocumentCommand {
            command: DocumentCommand::AddRegionBorder(
                AddRegionBorder {
                    points: region_borders,
                },
            ),
            win,
        }
        .invoke();

        format!("Loaded regions map image from {file_path}")
    };

    let message = match result
    {
        Ok(m) => m,
        Err(e) =>
        {
            format!("Error loading regions map image: {e}")
        },
    };

    SetStatus { message, win }.invoke();

    UpdatePreview { win }.invoke();
}
