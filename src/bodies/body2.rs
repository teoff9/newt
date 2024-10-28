//28.10.24 by Matteo Fava

use super::shape::Shape2D;
use crate::geom::{point::Point2D, vector2::Vector2D};

//Body2D struct
pub struct Body2D {
    pos: Point2D,
    shape: Shape2D,
    mass: f64,
    charge: f64,
    velocity: Vector2D,
    accelleration: Vector2D,
    is_static: bool
}