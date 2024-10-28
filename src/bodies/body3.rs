//28.10.24 by Matteo Fava

use super::shape::Shape3D;
use crate::geom::{point::Point3D, vector3::Vector3D};

//Body2D struct
pub struct Body3D {
    pos: Point3D,
    shape: Shape3D,
    mass: f64,
    charge: f64,
    velocity: Vector3D,
    accelleration: Vector3D,
    is_static: bool
}