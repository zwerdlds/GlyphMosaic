use super::DocumentPoint;
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    self,
    collections::{
        HashSet,
        VecDeque,
    },
    hash::Hash,
    iter,
    usize,
};

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

#[derive(Clone, PartialEq, Eq)]
enum MatMarker
{
    Unclaimed,
    Claimed,
}

struct DocumentRegionGenerator
{
    pub(super) reg_map: Vec<Vec<MatMarker>>,
    max_y: usize,
    max_x: usize,
    calculated_regions: Option<Vec<Vec<DocumentPoint>>>,
}
impl DocumentRegionGenerator
{
    pub(super) fn new(
        width: usize,
        height: usize,
        borders: &HashSet<DocumentPoint>,
    ) -> DocumentRegionGenerator
    {
        let reg_map =
            DocumentRegionGenerator::build_region_map(
                width, height,
            );

        let mut new = DocumentRegionGenerator {
            reg_map,
            max_x: width - 1,
            max_y: height - 1,
            calculated_regions: None,
        };

        new.mark_borders(
             borders
        );

        new
    }

  
    pub(super)  fn build_region_map(
        width: usize,
        height: usize,
    ) -> Vec<Vec<MatMarker>>
    {
        iter::repeat_with(|| {
            iter::repeat(MatMarker::Unclaimed)
                .take(height)
                .collect()
        })
        .take(width)
        .collect()
    }

   
    pub(super) fn mark_borders(
        &mut self,
        borders: &HashSet<DocumentPoint>,
    )
    {
        borders
            .into_iter()
            .map(|p| (p.x() as usize, p.y() as usize))
            .for_each(|(x, y)| {
                self.reg_map[x][y] = MatMarker::Claimed
            });
    }

  
    pub(super)  fn scan_next_unclaimed(
        &self,
        from: DocumentPoint,
    ) -> Option<DocumentPoint>
    {
        let mut next = Some(from);
        let max_x = self.reg_map.len();
        let max_y = self.reg_map[0].len();

        loop
        {
            if let Some(npt) = next
            {
                let nx = npt.x() as usize;
                let ny = npt.y() as usize;

                if self.reg_map[nx][ny]
                    == MatMarker::Unclaimed
                {
                    break Some(npt);
                }
                else
                {
                    next = {
                        if (nx + 1) < max_x
                        {
                            Some(
                                ((nx + 1) as usize, ny)
                                    .into(),
                            )
                        }
                        else
                        {
                            if (ny + 1) < max_y
                            {
                                Some(
                                    (0, (ny + 1) as usize)
                                        .into(),
                                )
                            }
                            else
                            {
                                None
                            }
                        }
                    };
                }
            }
            else
            {
                break None;
            }
        }
    }

    pub(super) fn claimed(
        &self,
        pt: &DocumentPoint,
    ) -> bool
    {
        self.reg_map[pt.x()][pt.y()] == MatMarker::Claimed
    }

    pub(super) fn claim_point(
        &mut self,
        pt: &DocumentPoint,
    )
    {
        self.reg_map[pt.x()][pt.y()] = MatMarker::Claimed;
    }

    pub(super) fn find_and_mark_region_from_unclaimed_point(
        &mut self,
        from: DocumentPoint,
    ) -> Vec<DocumentPoint>
    {
        let region_points = &mut vec![from];

        let from_status =
            &mut self.reg_map[from.x()][from.y()];

        if *from_status != MatMarker::Unclaimed
        {
            panic!("Specified point is not unclaimed");
        }

        self.claim_point(&from);

        let mut frontier: VecDeque<DocumentPoint> =
            region_points.clone().into();

        loop
        {
            if let Some(front_point) = frontier.pop_front()
            {
                let x = front_point.x();
                let y = front_point.y();

                if x > 0
                {
                    let left = (x - 1, y).into();

                    if !self.claimed(&left)
                    {
                        self.claim_point(&left);
                        frontier.push_back(left);
                        region_points.push(left);
                    }
                }

                if x < self.max_x
                {
                    let right = (x + 1, y).into();

                    if !self.claimed(&right)
                    {
                        self.claim_point(&right);
                        frontier.push_front(right);
                        region_points.push(right);
                    }
                }

                if y > 0
                {
                    let up = (x, y - 1).into();

                    if !self.claimed(&up)
                    {
                        self.claim_point(&up);
                        frontier.push_back(up);
                        region_points.push(up);
                    }
                }

                if y < self.max_y
                {
                    let down = (x, y + 1).into();

                    if !self.claimed(&down)
                    {
                        self.claim_point(&down);
                        frontier.push_back(down);
                        region_points.push(down);
                    }
                }
            }
            else
            {
                break;
            }
        }

        region_points.to_vec()
    }

    pub fn regions(&mut self) -> Vec<Vec<DocumentPoint>>
    {
        if None == self.calculated_regions
        {
            self.calculate_regions();
        }

        self.calculated_regions.clone().unwrap()
    }

   pub(super) fn calculate_regions(&mut self)
    {
        let mut next_start_opt = self
            .scan_next_unclaimed((0usize, 0usize).into());

        let regions = &mut Vec::new();

        loop
        {
            if let Some(next_start) = next_start_opt
            {
                let next_reg = 
                self.find_and_mark_region_from_unclaimed_point(next_start);

                regions.push(next_reg);

                next_start_opt = self.scan_next_unclaimed(next_start);
            }
            else
            {
                break;
            }
        }

        self.calculated_regions = Some(regions.to_vec());
    }
}

impl DocumentRegion
{
    pub fn calculate_regions(
        width: usize,
        height: usize,
        borders: HashSet<DocumentPoint>,
    ) -> Vec<Vec<DocumentPoint>>
    {
        DocumentRegionGenerator::new(
            width, height, &borders,
        )
        .regions()
        .clone()
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
    extern crate test;
    use std::collections::HashSet;
    use crate::document::{DocumentPoint, region::{DocumentRegionGenerator, MatMarker}};
    use super::DocumentRegion;

    #[test]
    pub fn scan_unclaimed()
    {
        let mut r = DocumentRegionGenerator::new(
            2, 2, &HashSet::new(),
        );

        assert_eq!(r.scan_next_unclaimed((0,0).into()).unwrap(), (0,0).into());
        assert_eq!(r.scan_next_unclaimed((0,0).into()).unwrap(), (0,0).into());

        r.reg_map[0][0] = MatMarker::Claimed;

        assert_eq!(r.scan_next_unclaimed((0,0).into()).unwrap(), (1,0).into());
        
        r.reg_map[1][0] = MatMarker::Claimed;
        assert_eq!(r.scan_next_unclaimed((0,0).into()).unwrap(), (0,1).into());

        r.reg_map[0][1] = MatMarker::Claimed;
        assert_eq!(r.scan_next_unclaimed((0,0).into()).unwrap(), (1,1).into());

        r.reg_map[1][1] = MatMarker::Claimed;
        assert_eq!(r.scan_next_unclaimed((0,0).into()), None);
    }

    #[test]
    pub fn single_region_unclaimed()
    {
        let mut r = DocumentRegionGenerator::new(
            2, 2, &HashSet::new(),
        );

        let reg = r.find_and_mark_region_from_unclaimed_point((0,0).into());
        
        assert_eq!(reg.len(),4);
        assert!(reg.contains(&(0,0).into()));
        assert!(reg.contains(&(1,0).into()));
        assert!(reg.contains(&(0,1).into()));
        assert!(reg.contains(&(1,1).into()));
    }

    #[test]
    pub fn claim_point()
    {
        let mut r = DocumentRegionGenerator::new(
            1, 1, &HashSet::new(),
        );

        r.claim_point(&(0,0).into());
        
        assert!(r.claimed(&(0,0).into()));
    }

    #[test]
    pub fn three_pixel_two_regions()
    {
        
        let regs = DocumentRegion::calculate_regions(
            3,
            1,
            [(1, 0)]
                .into_iter()
                .map(From::from)
                .collect(),
        );

        assert_eq!(2, regs.len());
    }

    #[test]
    pub fn single_region_generation()
    {
        let regs = DocumentRegion::calculate_regions(
            5,
            5,
            [].into_iter().collect(),
        );

        assert_eq!(1, regs.len());
    }

    #[test]
    pub fn two_regions_generation()
    {
        let regs = DocumentRegion::calculate_regions(
            5,
            5,
            [(3, 0), (3, 1), (3, 2), (3, 3), (3, 4)]
                .into_iter()
                .map(From::from)
                .collect(),
        );

        assert_eq!(2, regs.len());
    }

    #[test]
    pub fn four_regions_generation()
    {
        let regs = DocumentRegion::calculate_regions(
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
            .into_iter()
            .map(From::from)
            .collect(),
        );

        assert_eq!(4, regs.len());
    }

    #[test]
    pub fn two_medium_regions_generation()
    {
        let line = DocumentPoint::new((250, 0))
            .raster_line_to((250, 499).into());

        let regs = DocumentRegion::calculate_regions(
            500,
            500,
            line.into_iter().collect(),
        );

        assert_eq!(2, regs.len());
    }

    #[test]
    pub fn two_large_regions_generation()
    {
        let line = DocumentPoint::new((250, 0))
            .raster_line_to((250, 2999).into());

        let regs = DocumentRegion::calculate_regions(
            3000,
            3000,
            line.into_iter().collect(),
        );

        assert_eq!(2, regs.len());
    }

    #[test]
    pub fn region_disjoint()
    {
        let regs = DocumentRegion::calculate_regions(
            500,
            500,
            DocumentPoint::new((250, 0))
            .raster_line_to((250, 499).into()).into_iter().collect(),
        );


        let reg1 : HashSet<DocumentPoint> = regs[0].iter().map(ToOwned::to_owned).collect();
        let reg2 : HashSet<DocumentPoint> = regs[1].iter().map(ToOwned::to_owned).collect();

        assert!(reg1.is_disjoint(&reg2));
    }
}
