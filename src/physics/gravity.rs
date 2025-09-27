//05.11.24 by Matteo Fava
use crate::{bodies::body::Body, geom::vec::Vector, physics::constants::G};

pub fn grav_field(p: &Body, pos: &Vector) -> Vector {
    ( (*pos - *p.pos()) * *p.mass() * G ) / (-p.distance_squared(pos).powf(3.0/2.0))
}