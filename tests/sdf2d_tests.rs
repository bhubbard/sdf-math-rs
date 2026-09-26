use approx::assert_relative_eq;
use glam::Vec2;
use sdf_math_rs::sdf2d::*;

#[test]
fn test_circle_sdf() {
    let r = 2.0;
    // Center is inside: dist = -r
    assert_relative_eq!(circle(Vec2::ZERO, r), -2.0);
    // On the boundary: dist = 0
    assert_relative_eq!(circle(Vec2::new(2.0, 0.0), r), 0.0);
    assert_relative_eq!(circle(Vec2::new(0.0, -2.0), r), 0.0);
    // Outside: dist = 1
    assert_relative_eq!(circle(Vec2::new(3.0, 0.0), r), 1.0);
}

#[test]
fn test_box2d_sdf() {
    let b = Vec2::new(2.0, 1.0);
    // Origin inside
    assert_relative_eq!(box2d(Vec2::ZERO, b), -1.0);
    // Edge surface
    assert_relative_eq!(box2d(Vec2::new(2.0, 0.5), b), 0.0);
    // Corner surface
    assert_relative_eq!(box2d(Vec2::new(2.0, 1.0), b), 0.0);
    // Outside along x
    assert_relative_eq!(box2d(Vec2::new(4.0, 0.0), b), 2.0);
    // Diagonal outside corner (3, 2) -> dist from (2, 1) is sqrt(1^2 + 1^2) = sqrt(2)
    assert_relative_eq!(box2d(Vec2::new(3.0, 2.0), b), 2.0f32.sqrt());
}

#[test]
fn test_oriented_box_sdf() {
    let a = Vec2::new(-2.0, 0.0);
    let b = Vec2::new(2.0, 0.0);
    let th = 1.0;

    // Center point (0, 0) inside
    assert_relative_eq!(oriented_box(Vec2::ZERO, a, b, th), -0.5);
    // Edge
    assert_relative_eq!(oriented_box(Vec2::new(0.0, 0.5), a, b, th), 0.0);
    // Outside
    assert_relative_eq!(oriented_box(Vec2::new(0.0, 1.5), a, b, th), 1.0);
}

#[test]
fn test_segment_sdf() {
    let a = Vec2::new(0.0, 0.0);
    let b = Vec2::new(10.0, 0.0);

    // On segment
    assert_relative_eq!(segment(Vec2::new(5.0, 0.0), a, b), 0.0);
    // Perpendicular offset
    assert_relative_eq!(segment(Vec2::new(5.0, 3.0), a, b), 3.0);
    // Beyond endpoint a
    assert_relative_eq!(segment(Vec2::new(-4.0, 0.0), a, b), 4.0);
}

#[test]
fn test_capsule2d_sdf() {
    let a = Vec2::new(-3.0, 0.0);
    let b = Vec2::new(3.0, 0.0);
    let r = 1.0;

    // Center inside
    assert_relative_eq!(capsule2d(Vec2::ZERO, a, b, r), -1.0);
    // Boundary on side
    assert_relative_eq!(capsule2d(Vec2::new(0.0, 1.0), a, b, r), 0.0);
    // Outside cap
    assert_relative_eq!(capsule2d(Vec2::new(5.0, 0.0), a, b, r), 1.0);
}

#[test]
fn test_equilateral_triangle_sdf() {
    let r = 1.0;
    // Inside centroid
    assert!(equilateral_triangle(Vec2::ZERO, r) < 0.0);
    // Outside far away
    assert!(equilateral_triangle(Vec2::new(5.0, 5.0), r) > 0.0);
}
