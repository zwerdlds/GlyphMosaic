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
    Notebook,
};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(
    resource = "/me/zwerdlds/glyphmosaic/gui/gmwindow.ui"
)]
pub struct Window
{
    #[template_child]
    pub settings_notebook: TemplateChild<Notebook>,

    #[template_child]
    pub select_source: TemplateChild<Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window
{
    type ParentType = ApplicationWindow;
    type Type = super::Window;

    const NAME: &'static str = "GMMainWindow";

    fn class_init(c: &mut Self::Class)
    {
        c.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>)
    {
        obj.init_template();
    }
}

impl ObjectImpl for Window
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
impl WidgetImpl for Window
{
}

// Trait shared by all windows
impl WindowImpl for Window
{
}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window
{
}
