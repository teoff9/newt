use crate::sim::{
    config::{read_config, write_config, Bodies, Config},
    geom::Vec3,
    simulation::Simulation,
};

#[test]
fn test_read_config() {
    let expected = Config {
        g: 1.0,
        dt: 0.001,
        steps: 1000000,
        softening: 0.001,
        bodies: Bodies {
            pos: vec![[0.; 3], [1000.; 3], [500.0; 3]],
            vel: vec![[0.; 3]; 3],
            mass: vec![1.; 3],
        },
    };
    let res = read_config("src/data/test/input.json").unwrap();
    assert_eq!(res, expected);
}

#[test]
fn test_write_config() {
    let c = Config {
        g: 1.0,
        dt: 0.001,
        steps: 1000000,
        softening: 0.001,
        bodies: Bodies {
            pos: vec![[0.; 3], [1000.; 3], [500.0; 3]],
            vel: vec![[0.; 3]; 3],
            mass: vec![1.; 3],
        },
    };

    write_config("src/data/test/output.json", &c).unwrap();
    let w = read_config("src/data/test/output.json").unwrap();
    assert_eq!(w, c);
}

#[test]
fn save_simulation() {
    let sim1 = Simulation::from(
        1.,
        0.001,
        1000000,
        0.001,
        vec![
            Vec3::from([0.; 3]),
            Vec3::from([1000.; 3]),
            Vec3::from([500.; 3]),
        ],
        vec![Vec3::from([0.; 3]); 3],
        vec![1.; 3],
    );
    sim1.save_to_file("src/data/test/output.json").unwrap();
    let sim2 = Simulation::from_config_file("src/data/test/input.json").unwrap();
    assert_eq!(sim1, sim2);
}
