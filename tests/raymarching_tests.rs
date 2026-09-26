use approx::assert_relative_eq;
use glam::Vec3;
use sdf_math_rs::raymarching::*;
use sdf_math_rs::sdf3d::*;

#[test]
fn test_ray_march_sphere_hit() {
    let scene = |p: Vec3| sphere(p - Vec3::new(0.0, 0.0, 5.0), 1.0);

    let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0));
    let config = RayMarchConfig::default();

    let hit = ray_march(&ray, scene, &config);
    assert!(hit.hit);
    // Sphere is at z=5 with radius 1, so hit should be at z=4.0
    assert_relative_eq!(hit.distance, 4.0, epsilon = 1e-3);
    assert_relative_eq!(hit.position.z, 4.0, epsilon = 1e-3);
    // Normal facing back towards camera (-Z)
    assert_relative_eq!(hit.normal.z, -1.0, epsilon = 1e-2);
}

#[test]
fn test_ray_march_sphere_miss() {
    let scene = |p: Vec3| sphere(p - Vec3::new(0.0, 5.0, 5.0), 1.0);

    // Ray shooting along +Z misses sphere located up at y=5
    let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0));
    let config = RayMarchConfig::default();

    let hit = ray_march(&ray, scene, &config);
    assert!(!hit.hit);
}

#[test]
fn test_tetrahedron_normal_accuracy() {
    let scene = |p: Vec3| sphere(p, 2.0);

    // Test normal on +X face of sphere
    let pt = Vec3::new(2.0, 0.0, 0.0);
    let normal = estimate_normal(pt, &scene, 1e-4);
    assert_relative_eq!(normal.x, 1.0, epsilon = 1e-3);
    assert_relative_eq!(normal.y, 0.0, epsilon = 1e-3);
    assert_relative_eq!(normal.z, 0.0, epsilon = 1e-3);
}

#[test]
fn test_ambient_occlusion_and_shadow() {
    let scene = |p: Vec3| sphere(p, 1.0);

    // On top of sphere (0, 1, 0), normal is (0, 1, 0)
    let p = Vec3::new(0.0, 1.0, 0.0);
    let n = Vec3::Y;
    let ao = ambient_occlusion(p, n, &scene, 0.05, 5);
    // Unoccluded sphere surface should have high AO (close to 1.0)
    assert!(ao > 0.85);

    // Soft shadow test: light above looking down at point on top of sphere is fully lit
    let light_dir = Vec3::Y;
    let shadow = soft_shadow(p + n * 0.02, light_dir, 0.02, 10.0, 16.0, &scene);
    assert_relative_eq!(shadow, 1.0, epsilon = 0.05);

    // Light pointing through the sphere should be completely occluded (shadow = 0)
    let shadow_occluded = soft_shadow(Vec3::new(0.0, -2.0, 0.0), Vec3::Y, 0.02, 10.0, 16.0, &scene);
    assert_relative_eq!(shadow_occluded, 0.0, epsilon = 0.05);
}
