use gtk4::{
    prelude::FileExt,
    traits::FileChooserExt,
    FileChooserDialog,
};

pub fn get_dialog_path(
    dialog: &FileChooserDialog
) -> Result<String, String>
{
    dialog
        .file()
        .ok_or("Couldn't get file from filechooser.")?
        .path()
        .ok_or("Couldn't get file path.")?
        .to_str()
        .ok_or("Path not convertable to string.")
        .map(|p| p.to_string())
        .map_err(|e| e.to_string())
}
