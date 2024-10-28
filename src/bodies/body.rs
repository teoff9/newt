//28.10.24 by Matteo Fava
use super::shape::Shape;
use crate::geom::vec::Vector;

//Body2D struct
pub struct Body {
    pos: Vector,
    prev_pos: Vector,
    shape: Shape,
    mass: f64,
    charge: f64,
    accelleration: Vector,
    is_static: bool
}