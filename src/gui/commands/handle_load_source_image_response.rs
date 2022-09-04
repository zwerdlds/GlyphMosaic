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

#[must_use]
pub struct HandleLoadSourceImageResponse<'a>
{
    pub dialog: &'a FileChooserDialog,
    pub response: ResponseType,
    pub win: &'a DocumentWindow,
}

impl HandleLoadSourceImageResponse<'_>
{
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

            WindowDocumentCommand {
                command: DocumentCommand::SetSourceImage(
                    img.into(),
                ),
                win: self.win,
            }
            .invoke();

            format!("Loaded source image from {file_path}")
        };

        let message = match result
        {
            Ok(m) => m,
            Err(e) =>
            {
                format!("Error loading source image: {e}")
            },
        };

        SetStatus {
            message,
            win: self.win,
        }
        .invoke();
        QueuePreviewRefresh { win: self.win }.invoke();
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
