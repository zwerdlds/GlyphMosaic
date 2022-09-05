use gtk4::gio;

fn main()
{
    gio::compile_resources(
        "src/gui/resources",
        "src/gui/resources/resources.gresource.xml",
        "glyphmosaicgui.gresource",
    );
}
