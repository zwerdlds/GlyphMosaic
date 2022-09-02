use super::{
    QueuePreviewRefresh,
    SetStatus,
};
use crate::{
    commands::WindowDocumentCommand,
    document_window::DocumentWindow,
    util::get_dialog_path,
};
use glyph_mosaic::commands::DocumentCommand;
use gtk4::{
    gdk_pixbuf::Pixbuf,
    traits::GtkWindowExt,
    FileChooserDialog,
    ResponseType,
};

pub struct HandleLoadSourceImageResponse
{
    dialog: FileChooserDialog,
    response: ResponseType,
    win: DocumentWindow,
}

impl HandleLoadSourceImageResponse
{
    pub fn new(
        dialog: FileChooserDialog,
        response: ResponseType,
        win: DocumentWindow,
    ) -> HandleLoadSourceImageResponse
    {
        HandleLoadSourceImageResponse {
            dialog,
            response,
            win,
        }
    }

    pub fn invoke(self)
    {
        self.dialog.close();

        if self.response != ResponseType::Ok
        {
            return;
        }

        let result: Result<String, String> = try {
            let file_path = get_dialog_path(&self.dialog)?;

            let img = Pixbuf::from_file(file_path.clone())
                .map_err(|e| {
                    format!(
                        "Failed getting source image \
                         data: {e}"
                    )
                })?;

            WindowDocumentCommand::new(
                DocumentCommand::SetSourceImage(img.into()),
                self.win.clone(),
            )
            .invoke();

            format!("Loaded source image from {file_path}")
        };

        let result = result.map_err(|e| {
            format!("Loading source image failed: {e}")
        });

        SetStatus::new_from_result(
            result,
            self.win.clone(),
        )
        .invoke();
        QueuePreviewRefresh::new(self.win).invoke();
    }
}

#[cfg(test)]
mod tests
{
    use std::path::{
        Path,
        PathBuf,
    };

    #[test]
    fn validate_path_canonicalization()
    {
        let path = Path::new("./Cargo.toml");
        let canon = path.canonicalize().unwrap();

        assert_eq!(
            canon,
            PathBuf::from(
                "/mnt/Speedy/Development/GlyphMosaic/\
                 Cargo.toml"
            )
        );
    }
}
