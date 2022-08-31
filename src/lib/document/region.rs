use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::{
        Hash,
        Hasher,
    },
};

use serde::{
    Deserialize,
    Serialize,
};

use super::DocumentPoint;

#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Clone,
    Eq,
    Hash,
)]
pub struct DocumentRegion
{
    marker_point: DocumentPoint,
}

impl DocumentRegion
{
    pub fn calculate_region_points(
        width: u32,
        height: u32,
        borders: HashSet<DocumentPoint>,
    ) -> Vec<HashSet<DocumentPoint>>
    {
        let mut regions: Vec<HashSet<DocumentPoint>> =
            Vec::new();

        let non_border_pts: HashSet<DocumentPoint> = (0
            ..width)
            .map(|x| (0..height).map(move |y| (x, y)))
            .flatten()
            .map(From::from)
            .filter(|pt| !borders.contains(pt))
            .collect();

        let mut rem_pts = non_border_pts.clone();

        loop
        {
            if let Some(start) =
                rem_pts.iter().next().map(ToOwned::to_owned)
            {
                let mut region_pts: HashSet<DocumentPoint> =
                    HashSet::new();

                let mut front_pts: HashSet<DocumentPoint> =
                    HashSet::new();

                region_pts.insert(start);
                front_pts.insert(start);
                rem_pts.remove(&start);

                loop
                {
                    if let Some(f_pt) = front_pts
                        .iter()
                        .next()
                        .map(ToOwned::to_owned)
                    {
                        let f_pt_y = f_pt.y();
                        let f_pt_x = f_pt.x();

                        [
                            if f_pt_x < (width - 1)
                            {
                                Some(DocumentPoint::new(
                                    f_pt_x + 1,
                                    f_pt_y,
                                ))
                            }
                            else
                            {
                                None
                            },
                            if f_pt_x > 0
                            {
                                Some(DocumentPoint::new(
                                    f_pt_x - 1,
                                    f_pt_y,
                                ))
                            }
                            else
                            {
                                None
                            },
                            if f_pt_y < (height - 1)
                            {
                                Some(DocumentPoint::new(
                                    f_pt_x,
                                    f_pt_y + 1,
                                ))
                            }
                            else
                            {
                                None
                            },
                            if f_pt_y > 0
                            {
                                Some(DocumentPoint::new(
                                    f_pt_x,
                                    f_pt_y - 1,
                                ))
                            }
                            else
                            {
                                None
                            },
                        ]
                        .iter()
                        .map(ToOwned::to_owned)
                        .filter_map(|p| p)
                        .filter(|p| rem_pts.contains(p))
                        .for_each(
                            |pt| {
                                front_pts.insert(pt);
                            },
                        );

                        region_pts.insert(f_pt);
                        rem_pts.remove(&f_pt);
                        front_pts.remove(&f_pt);
                    }
                    else
                    {
                        break;
                    }
                }

                regions.push(region_pts);
            }
            else
            {
                break;
            }
        }

        regions
    }

    pub fn new(
        marker_point: DocumentPoint
    ) -> DocumentRegion
    {
        DocumentRegion { marker_point }
    }
}

impl From<DocumentPoint> for DocumentRegion
{
    fn from(pt: DocumentPoint) -> DocumentRegion
    {
        DocumentRegion::new(pt)
    }
}

impl From<&DocumentPoint> for DocumentRegion
{
    fn from(pt: &DocumentPoint) -> DocumentRegion
    {
        pt.to_owned().into()
    }
}

#[cfg(test)]
pub mod tests
{
    use super::DocumentRegion;

    #[test]
    pub fn validate_single_region_generation()
    {
        let regs = DocumentRegion::calculate_region_points(
            5,
            5,
            [].iter().map(From::from).collect(),
        );

        assert_eq!(1, regs.len());
    }

    #[test]
    pub fn validate_two_regions_generation()
    {
        let regs = DocumentRegion::calculate_region_points(
            5,
            5,
            [(3, 0), (3, 1), (3, 2), (3, 3), (3, 4)]
                .iter()
                .map(From::from)
                .collect(),
        );

        assert_eq!(2, regs.len());
    }

    #[test]
    pub fn validate_four_regions_generation()
    {
        let regs = DocumentRegion::calculate_region_points(
            5,
            5,
            [
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
                (0, 2),
                (1, 2),
                (2, 2),
                (4, 2),
            ]
            .iter()
            .map(From::from)
            .collect(),
        );

        assert_eq!(4, regs.len());
    }

    #[test]
    pub fn validate_region_updates()
    {
        todo!();
    }

    #[test]
    pub fn validate_region_disjoint()
    {
        todo!();
    }
}
