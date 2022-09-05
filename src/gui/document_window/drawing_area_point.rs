use delegate::delegate;
use glyph_mosaic::{
    document::DocumentPoint,
    point::GenericPoint,
};

#[derive(Debug, Clone, Copy)]
pub struct DrawingAreaPoint(GenericPoint<f64>);

impl DrawingAreaPoint
{
    delegate! {
        to self.0 {
            pub fn x(&self) -> f64;
            pub fn y(&self) -> f64;
        }
    }

    fn new(pt: (f64, f64)) -> DrawingAreaPoint
    {
        DrawingAreaPoint(GenericPoint::new(pt))
    }

    pub fn as_document_point(
        &self,
        zoom: f64,
    ) -> DocumentPoint
    {
        self.scale(1f64 / zoom).into()
    }

    pub fn scale(
        &self,
        scale: f64,
    ) -> DrawingAreaPoint
    {
        DrawingAreaPoint::new((
            (self.x() * scale),
            (self.y() * scale),
        ))
    }
}

impl From<DrawingAreaPoint> for DocumentPoint
{
    fn from(p: DrawingAreaPoint) -> Self
    {
        DocumentPoint::new((p.x() as usize, p.y() as usize))
    }
}

impl From<(f64, f64)> for DrawingAreaPoint
{
    fn from(p: (f64, f64)) -> Self
    {
        DrawingAreaPoint::new(p)
    }
}

impl From<DrawingAreaPoint> for Vec<DrawingAreaPoint>
{
    fn from(pt: DrawingAreaPoint) -> Self
    {
        vec![pt]
    }
}
