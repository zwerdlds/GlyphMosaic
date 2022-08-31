use super::DocumentCommand;
use crate::{
    document::DocumentPoint,
    prelude::Document,
};
use std::collections::HashSet;

pub struct AddRegionBorder
{
    pub points: HashSet<DocumentPoint>,
}

impl DocumentCommand for AddRegionBorder
{
    fn transform_document(
        self,
        doc: &Document,
    ) -> Document
    {
        let mut doc = doc.clone();

        let union: HashSet<DocumentPoint> = doc
            .region_border_pixels
            .union(&self.points)
            .map(|p| p.to_owned())
            .collect();

        doc.region_border_pixels = union;

        doc
    }
}

impl From<DocumentPoint> for AddRegionBorder
{
    fn from(pt: DocumentPoint) -> AddRegionBorder
    {
        let mut points = HashSet::new();

        points.insert(pt);

        AddRegionBorder { points }
    }
}

impl From<Vec<DocumentPoint>> for AddRegionBorder
{
    fn from(points: Vec<DocumentPoint>) -> AddRegionBorder
    {
        let points = points.into_iter().collect();
        AddRegionBorder { points }
    }
}
