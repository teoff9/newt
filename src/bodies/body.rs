//28.10.24 by Matteo Fava
use super::shape::Shape;
use crate::geom::vec::Vector;

//Body struct
#[derive(Debug)]
pub struct Body {
    pos: Vector,
    prev_pos: Vector,         //needed for verlet integration
    velocity: Option<Vector>, //needed to start sim
    shape: Shape,
    mass: f64,
    pub is_static: bool,
}

//IMPLEMENTATIONS
impl Body {
    //New instance of Body
    pub fn new(pos: Vector, v: Option<Vector>, shape: Shape, mass: f64, is_static: bool) -> Self {
        Self {
            pos,
            prev_pos: Vector::zero(),
            velocity: v,
            shape,
            mass,
            is_static,
        }
    }

    //Borrow pos
    pub fn pos(&self) -> &Vector {
        &self.pos
    }

    //Borrow prev
    pub fn prev(&self) -> &Vector {
        &self.prev_pos
    }

    //Update position and previous position
    pub fn move_to(&mut self, new_pos: Vector) {
        self.prev_pos = self.pos;
        self.pos = new_pos;
    }

    //Borrow mass
    pub fn mass(&self) -> &f64 {
        &self.mass
    }
}
