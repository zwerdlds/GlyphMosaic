use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Serialize, Deserialize, Debug, Clone, Hash, Copy,
)]
pub struct GenericPoint<T>
where
    T: Clone,
{
    x: T,
    y: T,
}

impl<T> GenericPoint<T>
where
    T: Copy,
{
    pub fn new((x, y): (T, T)) -> GenericPoint<T>
    {
        GenericPoint { x, y }
    }

    pub fn x(&self) -> T
    {
        self.x
    }

    pub fn y(&self) -> T
    {
        self.y
    }
}

impl<T> From<(T, T)> for GenericPoint<T>
where
    T: Copy,
{
    fn from(pt: (T, T)) -> Self
    {
        GenericPoint::new(pt)
    }
}

impl<T> PartialEq for GenericPoint<T>
where
    T: PartialEq,
    T: Clone,
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl<T> Eq for GenericPoint<T>
where
    T: PartialEq,
    T: Clone,
{
}
