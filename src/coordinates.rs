use std::ops;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coordinate
{
    // No X for now
    pub y: i32,
    pub z: i32
}

impl ops::Add<Coordinate> for Coordinate
{
    type Output = Coordinate;
    fn add(self, _rhs: Coordinate) -> Coordinate
    {
        Coordinate{y: self.y + _rhs.y,
                   z: self.z + _rhs.z}
   }
}

impl ops::AddAssign for Coordinate
{
    fn add_assign(&mut self, _rhs: Self)
    {
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

impl ops::Mul<i32> for Coordinate
{
    type Output = Coordinate;
    fn mul(self, rhs: i32) -> Self::Output
    {
        Coordinate{y: self.y * rhs,
                   z: self.z * rhs}
    }
}