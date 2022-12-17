use nannou::prelude::*;

/* STRUCT */
pub struct Cube{
    position: Point3,
    size: f32,
    theta: f32,
    direction: f32,
    geom: geom::Cuboid
}

/* IMPLEMENTATION */
impl Cube{
    pub fn new (position: Point3, size: f32, theta: f32, direction: f32) -> Self {
        let geom = geom::Cuboid::from_xyz_whd(position, pt3(size,size,size));
        Cube { position, size, theta, direction, geom }
    }
}
