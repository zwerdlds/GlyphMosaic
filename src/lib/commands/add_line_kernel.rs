use super::DocumentTransformable;
use crate::{
    document::DocumentPoint,
    prelude::Document,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct AddLineKernel
{
    pub points: HashSet<DocumentPoint>,
}

impl DocumentTransformable for AddLineKernel
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

impl From<DocumentPoint> for AddLineKernel
{
    fn from(pt: DocumentPoint) -> AddLineKernel
    {
        let mut points = HashSet::new();

        points.insert(pt);

        AddLineKernel { points }
    }
}

impl From<Vec<DocumentPoint>> for AddLineKernel
{
    fn from(points: Vec<DocumentPoint>) -> AddLineKernel
    {
        let points = points.into_iter().collect();
        AddLineKernel { points }
    }
}