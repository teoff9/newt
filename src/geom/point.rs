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

//Point 3D
pub struct Point3D {
    x: f64,
    y: f64,
    z: f64
}

impl Point for Point3D {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
}

impl Point3D {
    pub fn new(x: f64, y: f64, z:f64) -> Self {
        Self {x,y,z}
    }

    pub fn from(xyz: (f64, f64, f64)) -> Self {
        Self {x: xyz.0, y: xyz.1, z: xyz.2}
    }
    pub fn set_position(&mut self, xyz: (f64, f64, f64)) {
        self.x = xyz.0;
        self.y = xyz.1;
        self.z = xyz.2;
    }
}