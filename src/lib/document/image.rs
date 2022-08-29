use gtk4::{
    gdk_pixbuf::{
        Colorspace,
        Pixbuf,
    },
    glib::Bytes,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct DocumentImage
{
    #[serde(with = "PixbufDef")]
    pub pixbuf: Pixbuf,
}

impl PartialEq for DocumentImage
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        if self.pixbuf.has_alpha()
            != other.pixbuf.has_alpha()
        {
            return false;
        }

        if self.pixbuf.bits_per_sample()
            != other.pixbuf.bits_per_sample()
        {
            return false;
        }

        if self.pixbuf.width() != other.pixbuf.width()
        {
            return false;
        }

        if self.pixbuf.height() != other.pixbuf.height()
        {
            return false;
        }

        if self.pixbuf.rowstride()
            != other.pixbuf.rowstride()
        {
            return false;
        }

        if self.pixbuf.colorspace()
            != other.pixbuf.colorspace()
        {
            return false;
        }

        match (
            self.pixbuf.pixel_bytes(),
            other.pixbuf.pixel_bytes(),
        )
        {
            (None, _) | (_, None) => false,
            (Some(spb), Some(opb)) =>
            {
                spb.to_vec().eq(&opb.to_vec())
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "gtk4::gdk_pixbuf::Pixbuf")]
pub(super) struct PixbufDef
{
    #[serde(getter = "get_pixbuf_data")]
    pub(super) data: Vec<u8>,
    #[serde(getter = "Pixbuf::has_alpha")]
    pub(super) has_alpha: bool,
    #[serde(getter = "Pixbuf::bits_per_sample")]
    pub(super) bits_per_sample: i32,
    #[serde(getter = "Pixbuf::width")]
    pub(super) width: i32,
    #[serde(getter = "Pixbuf::height")]
    pub(super) height: i32,
    #[serde(getter = "Pixbuf::rowstride")]
    pub(super) rowstride: i32,
    #[serde(getter = "get_pixbuf_colorspace")]
    pub(super) colorspace: ColorspaceDef,
}

#[derive(Serialize, Deserialize)]
pub(super) enum ColorspaceDef
{
    Rgb,
    Unk(i32),
}

impl From<ColorspaceDef> for Colorspace
{
    fn from(def: ColorspaceDef) -> Colorspace
    {
        match def
        {
            ColorspaceDef::Rgb => Colorspace::Rgb,
            ColorspaceDef::Unk(i) =>
            {
                Colorspace::__Unknown(i)
            },
        }
    }
}

fn get_pixbuf_colorspace(pb: &Pixbuf) -> ColorspaceDef
{
    match pb.colorspace()
    {
        Colorspace::Rgb => ColorspaceDef::Rgb,
        Colorspace::__Unknown(i) => ColorspaceDef::Unk(i),
        c =>
        {
            panic!(
                "Colorspace {c} is not mapped to a \
                 serializable ColorspaceDef."
            )
        },
    }
}

fn get_pixbuf_data(pb: &Pixbuf) -> Vec<u8>
{
    pb.pixel_bytes().unwrap().to_vec()
}

impl From<PixbufDef> for Pixbuf
{
    fn from(def: PixbufDef) -> Pixbuf
    {
        Pixbuf::from_bytes(
            &Bytes::from(&def.data),
            def.colorspace.into(),
            def.has_alpha,
            def.bits_per_sample,
            def.width,
            def.height,
            def.rowstride,
        )
    }
}

impl From<Pixbuf> for DocumentImage
{
    fn from(p: Pixbuf) -> DocumentImage
    {
        DocumentImage::new(p)
    }
}

impl From<DocumentImage> for Pixbuf
{
    fn from(di: DocumentImage) -> Pixbuf
    {
        di.pixbuf
    }
}

impl DocumentImage
{
    pub fn new(pixbuf: Pixbuf) -> DocumentImage
    {
        DocumentImage { pixbuf }
    }
}

#[cfg(test)]
pub mod tests
{
    use super::{
        ColorspaceDef,
        PixbufDef,
    };
    use crate::document::image::DocumentImage;

    pub(crate) fn generate_test_img() -> DocumentImage
    {
        DocumentImage::new(
            PixbufDef {
                data: vec![255, 255, 255],
                has_alpha: false,
                bits_per_sample: 8,
                width: 1,
                height: 1,
                rowstride: 4,
                colorspace: ColorspaceDef::Rgb,
            }
            .into(),
        )
    }

    #[test]
    fn validate_simple_image_serialization()
    {
        let img = generate_test_img();

        let desr_ser_res = serde_json::from_str(
            &serde_json::to_string_pretty(&img).unwrap(),
        )
        .unwrap();

        assert_eq!(img, desr_ser_res);
    }
}
