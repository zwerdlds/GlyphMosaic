use std::{
    cell::RefCell,
    rc::Rc,
};

use glyph_mosaic::document::Document;
use gtk4::{
    glib::{
        self,
        subclass::InitializingObject,
    },
    prelude::*,
    subclass::prelude::*,
    ApplicationWindow,
    Button,
    CompositeTemplate,
    DrawingArea,
    Label,
    Notebook,
};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(
    resource = "/me/zwerdlds/glyphmosaic/gui/gmwindow.ui"
)]
pub struct DocumentWindow
{
    pub document: Rc<RefCell<Document>>,

    #[template_child]
    pub settings_notebook: TemplateChild<Notebook>,

    #[template_child]
    pub select_source: TemplateChild<Button>,

    #[template_child]
    pub status_label: TemplateChild<Label>,

    #[template_child]
    pub preview_area: TemplateChild<DrawingArea>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for DocumentWindow
{
    type ParentType = ApplicationWindow;
    type Type = super::DocumentWindow;

    const NAME: &'static str = "GMDocumentWindow";

    fn class_init(c: &mut Self::Class)
    {
        c.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>)
    {
        obj.init_template();
    }
}

impl ObjectImpl for DocumentWindow
{
    fn constructed(
        &self,
        obj: &Self::Type,
    )
    {
        self.parent_constructed(obj);
    }
}

// Trait shared by all widgets
impl WidgetImpl for DocumentWindow
{
}

// Trait shared by all windows
impl WindowImpl for DocumentWindow
{
}

// Trait shared by all application windows
impl ApplicationWindowImpl for DocumentWindow
{
}
