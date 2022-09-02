use glyph_mosaic::{
    self,
    commands::{
        DocumentCommand,
        DocumentTransformable,
    },
    document::DocumentPoint,
    prelude::Document,
};
use gtk4::gdk_pixbuf::Pixbuf;

#[derive(Debug, PartialEq)]
pub enum SettingsTab
{
    Sources,
    Regions,
    Lines,
    Points,
    Glyphs,
    Export,
}

pub struct Model
{
    document: Document,
    settings_tab: SettingsTab,
    last_drag_pos: Option<DocumentPoint>,
}

impl Default for Model
{
    fn default() -> Self
    {
        let document = Document::default();
        let settings_tab = SettingsTab::Sources;
        let last_drag_pos = None;

        Model {
            document,
            settings_tab,
            last_drag_pos,
        }
    }
}

impl Model
{
    pub(crate) fn settings_tab(&self) -> &SettingsTab
    {
        &self.settings_tab
    }

    pub(crate) fn apply_command(
        &mut self,
        cmd: DocumentCommand,
    )
    {
        let doc = cmd.transform_document(&self.document);
        self.document = doc;
    }

    pub(crate) fn set_last_drag_pos(
        &mut self,
        pos: Option<DocumentPoint>,
    )
    {
        self.last_drag_pos = pos;
    }

    pub(crate) fn last_drag_pos(
        &self
    ) -> Option<DocumentPoint>
    {
        self.last_drag_pos
    }

    pub(crate) fn set_settings_tab(
        &mut self,
        tab: SettingsTab,
    )
    {
        self.settings_tab = tab;
    }

    pub(crate) async fn create_source_preview_base(
        &self
    ) -> Result<Pixbuf, String>
    {
        self.document.render_source_image()
    }

    pub(crate) async fn create_preview(
        &self
    ) -> Result<Pixbuf, String>
    {
        use SettingsTab::*;

        match &self.settings_tab
        {
            Sources =>
            {
                self.create_source_preview_base().await
            },
            Regions =>
            {
                self.document.render_regions_image().await
            },
            Lines | Points | Glyphs | Export =>
            {
                Err(format!(
                    "Preview does not currently \
                     supporting this tab."
                ))
            },
        }
    }
}

#[cfg(test)]
mod tests
{
    use gtk4::gdk_pixbuf::{
        Colorspace,
        Pixbuf,
    };

    #[test]
    fn validate_image_crop_immutability()
    {
        let img: Pixbuf = Pixbuf::new(
            Colorspace::Rgb,
            false,
            8,
            512,
            512,
        )
        .unwrap();

        let subimg =
            img.new_subpixbuf(0, 0, 100, 100).unwrap();

        assert_eq!(subimg.width(), 100);
        assert_eq!(subimg.height(), 100);
        assert_eq!(img.height(), 512);
        assert_eq!(img.width(), 512);
    }
}
