use anyhow::Result;

use crate::sim::config::{read_config, write_config, Bodies, Config};

#[derive(Debug, PartialEq)]
pub struct Simulation {
    g: f64,
    dt: f64,
    steps: i64,
    softening: f64,
    pos: Vec<[f64; 3]>,
    vel: Vec<[f64; 3]>,
    mass: Vec<f64>,
}

impl Simulation {
    pub fn from_config(c: Config) -> Self {
        Self {
            g: c.g,
            dt: c.dt,
            steps: c.steps,
            softening: c.softening,
            pos: c.bodies.pos,
            vel: c.bodies.vel,
            mass: c.bodies.mass,
        }
    }

    pub fn from_config_file(path: &str) -> anyhow::Result<Self> {
        let conf = read_config(path)?;
        Ok(Self::from_config(conf))
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let c = Config {
            g: self.g,
            dt: self.dt,
            steps: self.steps,
            softening: self.softening,
            bodies: Bodies {
                pos: self.pos.clone(),
                vel: self.vel.clone(),
                mass: self.mass.clone(),
            },
        };
        write_config(path, &c)
    }

    pub fn from(
        g: f64,
        dt: f64,
        steps: i64,
        softening: f64,
        pos: Vec<[f64; 3]>,
        vel: Vec<[f64; 3]>,
        mass: Vec<f64>,
    ) -> Self {
        Self {
            g,
            dt,
            steps,
            softening,
            pos,
            vel,
            mass,
        }
    }

    pub fn run() {
        todo!()
    }
}
