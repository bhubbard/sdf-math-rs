use std::time::Instant;
use glam::{Vec2, Vec3};
use sdf_math_rs::ops::{smin, subtraction, union};
use sdf_math_rs::raymarching::{estimate_normal, ray_march, Ray, RayMarchConfig};
use sdf_math_rs::sdf2d::{circle, rounded_box2d};
use sdf_math_rs::sdf3d::{box3d, sphere, torus};

fn main() {
    println!("============================================================");
    println!("     sdf-math-rs (Rust) vs Inigo Quilez Reference (C/GLSL)  ");
    println!("============================================================");

    // 1. Primitive 3D SDF Queries Throughput
    println!("\n--- 1. Primitive 3D SDF Throughput (Sphere, Box, Torus) ---");
    {
        let iterations = 10_000_000;
        let start = Instant::now();
        let mut sum_dist = 0.0;

        for i in 0..iterations {
            let p = Vec3::new((i % 100) as f32 * 0.05, ((i / 100) % 100) as f32 * 0.05, 1.5);
            let d_sphere = sphere(p, 1.0);
            let d_box = box3d(p - Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.8, 0.8, 0.8));
            let d_torus = torus(p - Vec3::new(0.0, 2.0, 0.0), Vec2::new(1.2, 0.3));
            let d = smin(smin(d_sphere, d_box, 0.2), d_torus, 0.2);
            sum_dist += d;
        }

        let elapsed = start.elapsed();
        let ns_per_eval = elapsed.as_nanos() as f64 / iterations as f64;
        let evals_per_sec = iterations as f64 / elapsed.as_secs_f64();

        println!(
            "SDF Compound Evals: {} | Time: {:.2?} | Latency: {:.2} ns/eval | {:>10.0} evals/s | Sum: {:.1}",
            iterations, elapsed, ns_per_eval, evals_per_sec, sum_dist
        );
    }

    // 2. Full Sphere-Tracing Ray Marching (Raycast Resolution)
    println!("\n--- 2. Sphere Tracing Raymarching Engine (Complex CSG Scene) ---");
    {
        let scene = |p: Vec3| {
            let s1 = sphere(p - Vec3::new(0.0, 0.0, 5.0), 1.2);
            let b1 = box3d(p - Vec3::new(0.0, 0.0, 5.0), Vec3::splat(1.0));
            let blend = smin(s1, b1, 0.3);
            let hole = sphere(p - Vec3::new(0.0, 0.0, 5.0), 0.7);
            subtraction(hole, blend)
        };

        let config = RayMarchConfig {
            min_dist: 0.01,
            max_dist: 50.0,
            max_steps: 64,
            epsilon: 0.001,
        };

        let rays_to_trace = 500_000;
        let start = Instant::now();
        let mut hits = 0;

        for i in 0..rays_to_trace {
            let u = (i % 500) as f32 / 500.0 - 0.5;
            let v = (i / 500) as f32 / 1000.0 - 0.5;
            let dir = Vec3::new(u * 1.2, v * 1.2, 1.0).normalize();
            let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), dir);

            let hit = ray_march(&ray, scene, &config);
            if hit.hit {
                hits += 1;
            }
        }

        let elapsed = start.elapsed();
        let us_per_ray = elapsed.as_micros() as f64 / rays_to_trace as f64;
        let rays_per_sec = rays_to_trace as f64 / elapsed.as_secs_f64();

        println!(
            "Rays Marched: {} | Time: {:.2?} | Latency: {:.2} µs/ray | {:>10.0} rays/s | Hits: {}",
            rays_to_trace, elapsed, us_per_ray, rays_per_sec, hits
        );
    }

    // 3. Tetrahedron Normal Estimation
    println!("\n--- 3. Tetrahedron Analytical Surface Normal Estimation ---");
    {
        let scene = |p: Vec3| torus(p, Vec2::new(2.0, 0.5));
        let iterations = 2_000_000;
        let start = Instant::now();
        let mut normal_sum = Vec3::ZERO;

        for i in 0..iterations {
            let angle = (i % 360) as f32 * std::f32::consts::PI / 180.0;
            let p = Vec3::new(angle.cos() * 2.5, 0.0, angle.sin() * 2.5);
            let n = estimate_normal(p, &scene, 1e-4);
            normal_sum += n;
        }

        std::hint::black_box(normal_sum);
        let elapsed = start.elapsed();
        let ns_per_normal = elapsed.as_nanos() as f64 / iterations as f64;
        let normals_per_sec = iterations as f64 / elapsed.as_secs_f64();

        println!(
            "Normal Estimates: {} | Time: {:.2?} | Latency: {:.2} ns/normal | {:>10.0} normals/s | Sum: {:.1}",
            iterations, elapsed, ns_per_normal, normals_per_sec, normal_sum.length()
        );
    }

    // 4. 2D SDF Glyph & UI Primitives
    println!("\n--- 4. 2D SDF Glyph & UI Geometry (Rounded Box & Circle) ---");
    {
        let iterations = 10_000_000;
        let start = Instant::now();
        let mut sum_dist2d = 0.0;

        for i in 0..iterations {
            let p = Vec2::new((i % 200) as f32 * 0.1, ((i / 200) % 200) as f32 * 0.1);
            let d_box = rounded_box2d(p, Vec2::new(10.0, 5.0), 2.0);
            let d_circ = circle(p - Vec2::new(5.0, 2.0), 3.0);
            sum_dist2d += union(d_box, d_circ);
        }

        let elapsed = start.elapsed();
        let ns_per_eval2d = elapsed.as_nanos() as f64 / iterations as f64;
        let evals2d_per_sec = iterations as f64 / elapsed.as_secs_f64();

        println!(
            "2D SDF Evaluations: {} | Time: {:.2?} | Latency: {:.2} ns/eval ({:>10.0} evals/s) | Sum: {:.1}",
            iterations, elapsed, ns_per_eval2d, evals2d_per_sec, sum_dist2d
        );
    }

    println!("\n============================================================");
    println!("                      Benchmark Complete                    ");
    println!("============================================================");
}
