use approx::assert_relative_eq;
use glam::Vec3;
use sdf_math_rs::ops::*;

#[test]
fn test_csg_boolean_operations() {
    let d1 = 2.0;
    let d2 = -1.0;

    assert_relative_eq!(union(d1, d2), -1.0);
    assert_relative_eq!(intersection(d1, d2), 2.0);
    // Subtraction carves d1 out of d2: max(-d1, d2) = max(-2.0, -1.0) = -1.0
    assert_relative_eq!(subtraction(d1, d2), -1.0);
}

#[test]
fn test_polynomial_smooth_minimum() {
    let a = 1.0;
    let b = 1.0;
    let k = 0.5;

    // When a == b, smin(a, b, k) = a - k * 0.25 = 1.0 - 0.125 = 0.875
    let smoothed = smin(a, b, k);
    assert_relative_eq!(smoothed, 0.875);

    // Smooth min is always <= min(a, b)
    assert!(smoothed <= a.min(b));

    // When far apart (|a - b| >= k), smin equals exact min
    let far_a = 0.0;
    let far_b = 2.0;
    assert_relative_eq!(smin(far_a, far_b, k), 0.0);
}

#[test]
fn test_polynomial_smooth_maximum() {
    let a = 2.0;
    let b = 2.0;
    let k = 0.4;

    // When a == b, smax(a, b, k) = a + k * 0.25 = 2.0 + 0.1 = 2.1
    let smoothed = smax(a, b, k);
    assert_relative_eq!(smoothed, 2.1);
    assert!(smoothed >= a.max(b));
}

#[test]
fn test_domain_repetition() {
    let p = Vec3::new(7.2, 0.0, 0.0);
    let c = Vec3::new(2.0, 2.0, 2.0);

    let rep = repeat(p, c);
    // 7.2 / 2.0 = 3.6 -> round is 4.0 -> 7.2 - 2.0 * 4.0 = -0.8
    assert_relative_eq!(rep.x, -0.8, epsilon = 1e-5);
    assert_relative_eq!(rep.y, 0.0);
}

#[test]
fn test_twist_and_bend() {
    let p = Vec3::new(1.0, 0.0, 0.0);
    // Twist at y = 0 does nothing (angle = 0)
    let twisted = twist(p, 1.0);
    assert_relative_eq!(twisted.x, 1.0);
    assert_relative_eq!(twisted.z, 0.0);

    // Bend at x = 0 does nothing
    let p2 = Vec3::new(0.0, 1.0, 0.0);
    let bent = bend(p2, 1.0);
    assert_relative_eq!(bent.y, 1.0);
}

#[test]
fn test_displacement_noise() {
    let p = Vec3::new(0.0, 1.0, 2.0);
    // At x=0, sin(0) = 0 so noise is 0.0
    assert_relative_eq!(displacement_noise(p, 1.0, 0.5), 0.0);
}
