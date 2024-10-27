//27.10.24 by Matteo Fava

//Common trait for points
trait Point {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64 {
        0.0
    }
}

//Struct Point2D
#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point for Point2D {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x,y}
    }

    pub fn from(xy: (f64, f64)) -> Self {
        Self {x: xy.0, y: xy.1}
    }
    pub fn set_position(&mut self, xy: (f64, f64)) {
        self.x = xy.0;
        self.y = xy.1;
    }
}