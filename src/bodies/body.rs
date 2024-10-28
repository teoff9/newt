//28.10.24 by Matteo Fava
use super::shape::Shape;
use crate::geom::vec::Vector;

//Body2D struct
#[derive(Debug)]
pub struct Body {
    pos: Vector,
    prev_pos: Vector, //needed for verlet integration
    shape: Shape,
    mass: f64,
    charge: f64,
    accelleration: Vector,
    is_static: bool
}

//IMPLEMENTATIONS
impl Body {
    pub fn new(pos: Vector, shape: Shape, mass: f64, charge: f64, is_static: bool) -> Self {
        Self {
            pos,
            prev_pos: Vector::zero(),
            shape,
            mass,
            charge,
            accelleration: Vector::zero(),
            is_static
        }
    }
    pub fn pos(&self) -> &Vector {
        &self.pos
    }
    pub fn set_prev(&mut self, new_prev: Vector) {
        self.prev_pos = new_prev;
    }
    pub fn update_pos(&mut self, new_pos: Vector) {
        self.pos = new_pos;
    }
    //this method also set the previous pos
    pub fn move_to(&mut self, new_pos: Vector) {
        self.prev_pos = self.pos;
        self.pos = new_pos;
    }
    
}