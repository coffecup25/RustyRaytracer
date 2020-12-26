use vector::Vector3;

use super::vector;
pub struct Ray{
    pub origin: vector::Vector3,
    pub direction: vector::Vector3,
}

impl Ray{
    pub fn new(origin:Vector3,direction:Vector3)->Ray{
        Ray{
            origin,
            direction
        }
    }

    pub fn point_at(&self,t:f64)->vector::Vector3{
        self.origin+t*self.direction
    }
}