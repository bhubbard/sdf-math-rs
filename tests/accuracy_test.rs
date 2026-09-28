//! Rigorous Mathematical Accuracy & Geometric Parity Tests
//! Compares numerical Signed Distance Field calculations against analytical Euclidean ground-truth.

use approx::assert_relative_eq;
use glam::Vec3;
use sdf_math_rs::{
    estimate_normal, ops, ray_march, sdf2d, sdf3d, Ray, RayMarchConfig,
};

#[test]
fn test_sphere_sdf_analytical_exactness() {
    let radius = 2.5f32;
    let points = [
        (Vec3::new(0.0, 0.0, 0.0), -radius),
        (Vec3::new(2.5, 0.0, 0.0), 0.0),
        (Vec3::new(0.0, 5.0, 0.0), 2.5),
        (Vec3::new(3.0, 4.0, 0.0), 5.0 - radius),
        (Vec3::new(1.0, 2.0, 2.0), 3.0 - radius),
    ];

    for (p, expected) in points {
        let d = sdf3d::sphere(p, radius);
        assert_relative_eq!(d, expected, epsilon = 1e-6);
    }
}

#[test]
fn test_tetrahedron_normal_analytical_parity() {
    // For a sphere centered at origin, the true unit normal is analytically p / |p|
    let radius = 1.75f32;
    let scene = |p: Vec3| sdf3d::sphere(p, radius);

    let test_dirs = [
        Vec3::new(1.0, 0.0, 0.0).normalize(),
        Vec3::new(0.0, 1.0, 0.0).normalize(),
        Vec3::new(0.0, 0.0, 1.0).normalize(),
        Vec3::new(1.0, 1.0, 1.0).normalize(),
        Vec3::new(-2.0, 3.5, 1.2).normalize(),
        Vec3::new(0.3, -0.8, 1.5).normalize(),
    ];

    for dir in test_dirs {
        let surface_pt = dir * radius;
        let estimated_normal = estimate_normal(surface_pt, &scene, 1e-4);
        let expected_normal = dir;

        let dot = estimated_normal.dot(expected_normal);
        let diff = (estimated_normal - expected_normal).length();

        assert!(
            dot > 0.9999,
            "Normal orientation mismatch: dot={dot}, expected > 0.9999"
        );
        assert!(
            diff < 1e-3,
            "Normal vector L2 error exceeds threshold: diff={diff} < 1e-3"
        );
    }
}

#[test]
fn test_raymarch_sphere_tracing_precision() {
    let radius = 1.0f32;
    let center = Vec3::new(0.0, 0.0, 5.0);
    let scene = |p: Vec3| sdf3d::sphere(p - center, radius);
    let config = RayMarchConfig {
        min_dist: 0.01,
        max_dist: 100.0,
        max_steps: 128,
        epsilon: 1e-5,
    };

    let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0));
    let hit = ray_march(&ray, scene, &config);

    assert!(hit.hit, "Ray must hit sphere");
    let expected_t = 5.0 - radius; // Distance to outer boundary = 4.0
    assert_relative_eq!(hit.distance, expected_t, epsilon = 1e-4);

    let expected_pos = Vec3::new(0.0, 0.0, expected_t);
    assert_relative_eq!(hit.position.x, expected_pos.x, epsilon = 1e-4);
    assert_relative_eq!(hit.position.y, expected_pos.y, epsilon = 1e-4);
    assert_relative_eq!(hit.position.z, expected_pos.z, epsilon = 1e-4);
}

#[test]
fn test_csg_boolean_exact_metric_conservation() {
    let p = Vec3::new(1.2, 0.8, -0.4);
    let d1 = sdf3d::sphere(p, 1.0);
    let d2 = sdf3d::box3d(p, Vec3::new(0.8, 0.8, 0.8));

    // Union: exact min
    let u = ops::union(d1, d2);
    assert_eq!(u, d1.min(d2));

    // Intersection: exact max
    let i = ops::intersection(d1, d2);
    assert_eq!(i, d1.max(d2));

    // Subtraction: carves shape 1 out of shape 2: max(-d1, d2)
    let s = ops::subtraction(d1, d2);
    assert_eq!(s, (-d1).max(d2));
}

#[test]
fn test_2d_circle_euclidean_parity() {
    let r = 3.5f32;
    let pts = [
        (glam::Vec2::new(0.0, 0.0), -r),
        (glam::Vec2::new(3.5, 0.0), 0.0),
        (glam::Vec2::new(0.0, 7.0), 3.5),
        (glam::Vec2::new(3.0, 4.0), 5.0 - r),
    ];

    for (p, expected) in pts {
        let d = sdf2d::circle(p, r);
        assert_relative_eq!(d, expected, epsilon = 1e-6);
    }
}
