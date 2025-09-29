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
