use crate::point::GenericPoint;
use delegate::delegate;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Hash,
    Copy,
    PartialEq,
    Serialize,
    Eq,
    Deserialize,
)]
pub struct DocumentPoint(GenericPoint<usize>);

impl DocumentPoint
{
    delegate! {
        to self.0 {
            pub fn x(&self) -> usize;
            pub fn y(&self) -> usize;
        }
    }

    pub fn new(pt: (usize, usize)) -> DocumentPoint
    {
        DocumentPoint(GenericPoint::new(pt))
    }

    pub fn raster_line_to(
        &self,
        to: DocumentPoint,
    ) -> Vec<DocumentPoint>
    {
        let t_x = to.x() as i32;
        let t_y = to.y() as i32;
        let f_x = self.x() as i32;
        let f_y = self.y() as i32;

        let dx = t_x - f_x;
        let dy = t_y - f_y;

        let dx_f64 = dx as f64;
        let dy_f64 = dy as f64;

        let dx_abs = dx.abs() as u32;
        let dy_abs = dy.abs() as u32;

        let n_pts = dx_abs.max(dy_abs);

        let n_pts_f64 = n_pts as f64;

        let x_r = dx_f64 / n_pts_f64;
        let y_r = dy_f64 / n_pts_f64;

        (0..=n_pts)
            .map(|n| n as f64)
            .map(|n| ((n * x_r), (n * y_r)))
            .map(|(x_o, y_o)| (x_o.round(), y_o.round()))
            .map(|(x_o, y_o)| ((x_o as i32), (y_o as i32)))
            .map(|(x_o, y_o)| ((f_x + x_o), (f_y + y_o)))
            .map(|(x, y)| (x as usize, y as usize))
            .map(From::from)
            .collect()
    }
}

impl From<(usize, usize)> for DocumentPoint
{
    fn from(pt: (usize, usize)) -> DocumentPoint
    {
        DocumentPoint::new(pt)
    }
}

impl From<DocumentPoint> for Vec<DocumentPoint>
{
    fn from(pt: DocumentPoint) -> Vec<DocumentPoint>
    {
        vec![pt]
    }
}

#[cfg(test)]
pub mod tests
{
    use super::DocumentPoint;

    #[test]
    pub fn generate_point_coercion()
    {
        let cp: DocumentPoint = (0, 0).into();

        assert_eq!(0, cp.x());
        assert_eq!(0, cp.y());

        let cp: DocumentPoint = (5, 10).into();

        assert_eq!(5, cp.x());
        assert_eq!(10, cp.y());
    }

    #[test]
    fn simple_point_interpolation()
    {
        let res = DocumentPoint::new((0, 0))
            .raster_line_to((1, 1).into());

        assert_eq!(2, res.len());
        assert_eq!(res[0], (0, 0).into());
        assert_eq!(res[1], (1, 1).into());
    }

    #[test]
    fn eq_vert_point_interpolation()
    {
        let res = DocumentPoint::new((0, 0))
            .raster_line_to((0, 10).into());

        assert_eq!(11, res.len());
        assert_eq!(res[0], (0, 0).into());
        assert_eq!(res[5], (0, 5).into());
        assert_eq!(res[10], (0, 10).into());
    }

    #[test]
    fn eq_horiz_point_interpolation()
    {
        let res = DocumentPoint::new((0, 0))
            .raster_line_to((10, 0).into());

        assert_eq!(11, res.len());
        assert_eq!(res[0], (0, 0).into());
        assert_eq!(res[5], (5, 0).into());
        assert_eq!(res[10], (10, 0).into());
    }

    #[test]
    fn eq_dr_diagonal_point_interpolation()
    {
        let res = DocumentPoint::new((0, 0))
            .raster_line_to((10, 10).into());

        assert_eq!(11, res.len());
        assert_eq!(res[0], (0, 0).into());
        assert_eq!(res[10], (10, 10).into());
    }

    #[test]
    fn eq_dl_diagonal_point_interpolation()
    {
        let res = DocumentPoint::new((10, 0))
            .raster_line_to((0, 10).into());

        assert_eq!(11, res.len());
        assert_eq!(res[0], (10, 0).into());
        assert_eq!(res[5], (5, 5).into());
        assert_eq!(res[10], (0, 10).into());
    }

    #[test]
    fn eq_ul_diagonal_point_interpolation()
    {
        let res = DocumentPoint::new((10, 10))
            .raster_line_to((0, 0).into());

        assert_eq!(11, res.len());
        assert_eq!(res[0], (10, 10).into());
        assert_eq!(res[5], (5, 5).into());
        assert_eq!(res[10], (0, 0).into());
    }

    #[test]
    fn eq_ur_diagonal_point_interpolation()
    {
        let res = DocumentPoint::new((0, 10))
            .raster_line_to((10, 0).into());

        assert_eq!(11, res.len());
        assert_eq!(res[0], (0, 10).into());
        assert_eq!(res[5], (5, 5).into());
        assert_eq!(res[10], (10, 0).into());
    }

    #[test]
    fn right_inflection_for_downward_point_interpolation()
    {
        let res = DocumentPoint::new((0, 0))
            .raster_line_to((1, 10).into());

        assert_eq!(11, res.len());
        assert_eq!(res[0], (0, 0).into());
        assert_eq!(res[10], (1, 10).into());
        assert_eq!(res[4], (0, 4).into());
        assert_eq!(res[5], (1, 5).into());
    }

    #[test]
    fn down_inflection_for_rightward_point_interpolation()
    {
        let res = DocumentPoint::new((0, 0))
            .raster_line_to((10, 1).into());

        assert_eq!(11, res.len());
        assert_eq!(res[0], (0, 0).into());
        assert_eq!(res[10], (10, 1).into());
        assert_eq!(res[4], (4, 0).into());
        assert_eq!(res[5], (5, 1).into());
    }

    #[test]
    fn down_inflection_for_leftward_point_interpolation()
    {
        let res = DocumentPoint::new((10, 0))
            .raster_line_to((0, 1).into());

        assert_eq!(11, res.len());
        assert_eq!(res[0], (10, 0).into());
        assert_eq!(res[10], (0, 1).into());
        assert_eq!(res[4], (6, 0).into());
        assert_eq!(res[5], (5, 1).into());
    }

    #[test]
    fn right_inflection_for_upward_point_interpolation()
    {
        let res = DocumentPoint::new((0, 10))
            .raster_line_to((1, 0).into());

        assert_eq!(11, res.len());
        assert_eq!(res[0], (0, 10).into());
        assert_eq!(res[10], (1, 0).into());
        assert_eq!(res[4], (0, 6).into());
        assert_eq!(res[5], (1, 5).into());
    }
}
