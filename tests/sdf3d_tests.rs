use approx::assert_relative_eq;
use glam::{Vec2, Vec3};
use sdf_math_rs::sdf3d::*;

#[test]
fn test_sphere_sdf() {
    let r = 1.5;
    assert_relative_eq!(sphere(Vec3::ZERO, r), -1.5);
    assert_relative_eq!(sphere(Vec3::new(1.5, 0.0, 0.0), r), 0.0);
    assert_relative_eq!(sphere(Vec3::new(0.0, 0.0, 3.5), r), 2.0);
}

#[test]
fn test_box3d_sdf() {
    let b = Vec3::new(1.0, 2.0, 3.0);
    assert_relative_eq!(box3d(Vec3::ZERO, b), -1.0);
    assert_relative_eq!(box3d(Vec3::new(1.0, 1.0, 1.0), b), 0.0);
    assert_relative_eq!(box3d(Vec3::new(3.0, 0.0, 0.0), b), 2.0);
}

#[test]
fn test_rounded_box_sdf() {
    let b = Vec3::new(1.0, 1.0, 1.0);
    let r = 0.2;
    assert!(rounded_box(Vec3::ZERO, b, r) < 0.0);
    assert_relative_eq!(rounded_box(Vec3::new(1.0, 0.0, 0.0), b, r), 0.0);
}

#[test]
fn test_torus_sdf() {
    let t = Vec2::new(3.0, 1.0); // Major R=3, minor r=1
    // Center point (0, 0, 0) is outside the tube (dist = 3 - 1 = 2)
    assert_relative_eq!(torus(Vec3::ZERO, t), 2.0);
    // Point on the tube centerline at (3, 0, 0) is inside by minor radius 1.0
    assert_relative_eq!(torus(Vec3::new(3.0, 0.0, 0.0), t), -1.0);
    // Point on outer edge (4, 0, 0) is on boundary
    assert_relative_eq!(torus(Vec3::new(4.0, 0.0, 0.0), t), 0.0);
}

#[test]
fn test_capped_cylinder_sdf() {
    let h = 2.0; // half height
    let r = 1.0; // radius
    assert_relative_eq!(capped_cylinder(Vec3::ZERO, h, r), -1.0);
    assert_relative_eq!(capped_cylinder(Vec3::new(1.0, 0.0, 0.0), h, r), 0.0);
    assert_relative_eq!(capped_cylinder(Vec3::new(0.0, 2.0, 0.0), h, r), 0.0);
    assert_relative_eq!(capped_cylinder(Vec3::new(0.0, 4.0, 0.0), h, r), 2.0);
}

#[test]
fn test_capsule3d_sdf() {
    let a = Vec3::new(0.0, -2.0, 0.0);
    let b = Vec3::new(0.0, 2.0, 0.0);
    let r = 0.5;
    assert_relative_eq!(capsule3d(Vec3::ZERO, a, b, r), -0.5);
    assert_relative_eq!(capsule3d(Vec3::new(0.5, 0.0, 0.0), a, b, r), 0.0);
    assert_relative_eq!(capsule3d(Vec3::new(0.0, 3.5, 0.0), a, b, r), 1.0);
}

#[test]
fn test_plane_sdf() {
    let n = Vec3::Y;
    let h = 0.0;
    assert_relative_eq!(plane(Vec3::new(10.0, 5.0, -3.0), n, h), 5.0);
    assert_relative_eq!(plane(Vec3::new(0.0, -2.0, 0.0), n, h), -2.0);
}

#[test]
fn test_cone_sdf() {
    let r = 2.0;
    let h = 4.0;
    // Inside cone at y = -2.0, x=0, z=0
    assert!(cone(Vec3::new(0.0, -2.0, 0.0), r, h) < 0.0);
    // Apex at (0, 0, 0) is boundary
    assert_relative_eq!(cone(Vec3::ZERO, r, h), 0.0, epsilon = 1e-4);
    // Above apex is outside
    assert!(cone(Vec3::new(0.0, 2.0, 0.0), r, h) > 0.0);
}
