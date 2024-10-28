//28.10.24 by Matteo Fava


//Shape in 2D
pub enum Shape2D {
    Circle {r: f64},
    Rectangle {b: f64, h: f64},
    MaterialPoint
}

//Shape in 3D
pub enum Shape3D {
    Sphere{r:f64}
}