//27.10.24 by Matteo Fava

//trait Vector
pub trait Vector {
    fn vx(&self) -> f64;
    fn vy(&self) -> f64;
    fn vz(&self) -> f64 {
        0.0
    }
}