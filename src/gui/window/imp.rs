use gtk4::{
    glib::{self, subclass::InitializingObject},
    prelude::*,
    subclass::prelude::*,
    ApplicationWindow, Button, CompositeTemplate, Notebook,
};
// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(
    resource = "/me/zwerdlds/glyphmosaic/gui/gmwindow.ui"
)]
pub struct Window {
    #[template_child]
    pub settings_notebook: TemplateChild<Notebook>,

    #[template_child]
    pub select_source: TemplateChild<Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "GMMainWindow";
    type Type = super::Window;
    type ParentType = ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(
        &self,
        obj: &Self::Type,
    ) {
        self.parent_constructed(obj);

        print!("Window constructed.");

        self.select_source.connect_clicked(move |_| {
            print!("select source clicked.");
        });

        self.settings_notebook.connect_change_current_page(
            move |_, i| {
                print!("Settings page changing ({i}).");
                true
            },
        );
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
