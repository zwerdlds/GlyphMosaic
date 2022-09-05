mod render;
use self::render::RenderJoinHandle;
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
pub use render::RenderHandle;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SettingsTab
{
    Sources,
    Regions,
    Lines,
    Points,
    Glyphs,
    Export,
}

pub(crate) struct Model
{
    document: Document,
    settings_tab: SettingsTab,
    last_drag_pos: Option<DocumentPoint>,
    tokio_runtime: tokio::runtime::Runtime,
    current_render: Option<RenderJoinHandle>,
    current_render_handle: Option<RenderHandle>,
    current_preview: Option<Pixbuf>,
}

impl Default for Model
{
    fn default() -> Self
    {
        let document = Document::default();
        let settings_tab = SettingsTab::Sources;
        let last_drag_pos = None;
        let tokio_runtime =
            tokio::runtime::Runtime::new().unwrap();
        let current_render = None;
        let current_render_handle = None;
        let current_preview = None;

        Model {
            current_preview,
            current_render,
            tokio_runtime,
            document,
            current_render_handle,
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

    pub(crate) fn document(&self) -> &Document
    {
        &self.document
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
