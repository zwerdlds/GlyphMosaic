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
pub struct DocumentImage
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
struct PixbufDef
{
    #[serde(getter = "get_pixbuf_data")]
    data: Vec<u8>,
    #[serde(getter = "Pixbuf::has_alpha")]
    has_alpha: bool,
    #[serde(getter = "Pixbuf::bits_per_sample")]
    bits_per_sample: i32,
    #[serde(getter = "Pixbuf::width")]
    width: i32,
    #[serde(getter = "Pixbuf::height")]
    height: i32,
    #[serde(getter = "Pixbuf::rowstride")]
    rowstride: i32,
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
            Colorspace::Rgb,
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
mod tests
{
    use gtk4::gdk_pixbuf::Pixbuf;

    use crate::document::doc_img::DocumentImage;

    #[test]
    fn validate_simple_image_serialization()
    {
        let img: DocumentImage =
            Pixbuf::from_file("./test resources/1x1.png")
                .unwrap()
                .into();

        let serialize_res =
            serde_json::to_string_pretty(&img).unwrap();

        print!("{:?}", serialize_res);

        assert_eq!(
            serialize_res,
            "{\n  \"pixbuf\": {\n    \"data\": [\n      255,\n      255,\n      255\n    ],\n    \"has_alpha\": false,\n    \"bits_per_sample\": 8,\n    \"width\": 1,\n    \"height\": 1,\n    \"rowstride\": 4\n  }\n}"        );
    }

    #[test]
    fn validate_simple_document_load_json()
    {
        let img = include_str!(
            "../../../test resources/testimg.json"
        );

        let load_res: DocumentImage =
            serde_json::from_str(img).unwrap();

        let pb = load_res.pixbuf.pixel_bytes().unwrap();

        assert_eq!(pb.len(), 3);
        assert_eq!(pb[0], 255);
        assert_eq!(pb[1], 255);
        assert_eq!(pb[2], 255);
    }
}
