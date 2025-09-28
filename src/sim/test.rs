#[cfg(test)]
mod test_read_save {
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
        let res = read_config("src/test_data/input.json").unwrap();
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

        write_config("src/test_data/output.json", &c).unwrap();
        let w = read_config("src/test_data/output.json").unwrap();
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
        sim1.save_to_file("src/test_data/input.json").unwrap();
        let sim2 = Simulation::from_config_file("src/test_data/input.json").unwrap();
        assert_eq!(sim1, sim2);
    }
}

#[cfg(test)]
mod test_geom {
    use crate::sim::geom::Vec3;

    #[test]
    fn test_creation_and_access() {
        let v = Vec3::from([1.0, 2.0, 3.0]);
        assert_eq!(v.to_array(), [1.0, 2.0, 3.0]);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);

        let mut w = Vec3::zero();
        assert_eq!(w.to_array(), [0.0, 0.0, 0.0]);

        w[0] = 5.0;
        w[1] = 6.0;
        w[2] = 7.0;
        assert_eq!(w.to_array(), [5.0, 6.0, 7.0]);
    }

    #[test]
    fn test_add_and_sub() {
        let v1 = Vec3::from([1.0, 2.0, 3.0]);
        let v2 = Vec3::from([4.0, 5.0, 6.0]);

        let v3 = v1 + v2;
        assert_eq!(v3.to_array(), [5.0, 7.0, 9.0]);

        let v4 = v3 - v1;
        assert_eq!(v4.to_array(), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_add_assign_and_sub_assign() {
        let mut v = Vec3::from([1.0, 2.0, 3.0]);
        v += Vec3::from([4.0, 5.0, 6.0]);
        assert_eq!(v.to_array(), [5.0, 7.0, 9.0]);

        v -= Vec3::from([1.0, 1.0, 1.0]);
        assert_eq!(v.to_array(), [4.0, 6.0, 8.0]);
    }

    #[test]
    fn test_mul_and_mul_assign() {
        let mut v = Vec3::from([1.0, 2.0, 3.0]);
        let w = v * 2.0;
        assert_eq!(w.to_array(), [2.0, 4.0, 6.0]);

        v *= 3.0;
        assert_eq!(v.to_array(), [3.0, 6.0, 9.0]);

        let u = Vec3::from([2.0, 3.0, 4.0]);
        let mut z = Vec3::from([1.0, 2.0, 3.0]);
        z *= u;
        assert_eq!(z.to_array(), [2.0, 6.0, 12.0]);
    }

    #[test]
    fn test_div_and_div_assign() {
        let mut v = Vec3::from([2.0, 4.0, 6.0]);
        let w = v / 2.0;
        assert_eq!(w.to_array(), [1.0, 2.0, 3.0]);

        v /= 2.0;
        assert_eq!(v.to_array(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_abs() {
        let v = Vec3::from([3.0, 4.0, 12.0]);
        assert_eq!(v.abs(), 13.0); // √(9+16+144) = √169 = 13
    }
}
