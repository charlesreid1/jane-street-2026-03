use rand::{Rng, RngExt};
use std::f64::consts::PI;

/// Generate a uniformly random point on S^2.
fn random_point_on_sphere(rng: &mut impl Rng) -> [f64; 3] {
    let cos_theta: f64 = rng.random_range(-1.0_f64..=1.0);
    let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
    let phi: f64 = rng.random_range(0.0..2.0 * PI);
    [sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta]
}

/// Check if n points on S^2 all lie in some open hemisphere.
///
/// For continuous random points, we check all candidate hemispheres:
/// - Hemispheres centered at each point
/// - Hemispheres whose bounding great circle passes through each pair
fn in_some_hemisphere(points: &[[f64; 3]]) -> bool {
    let n = points.len();

    // Check hemisphere centered at each point
    for i in 0..n {
        if points.iter().all(|p| dot(&points[i], p) > 0.0) {
            return true;
        }
    }

    // Check hemispheres whose bounding great circle passes through pairs
    for i in 0..n {
        for j in (i + 1)..n {
            let normal = cross(&points[i], &points[j]);
            let norm = dot(&normal, &normal).sqrt();
            if norm < 1e-12 {
                continue;
            }
            let normal = [normal[0] / norm, normal[1] / norm, normal[2] / norm];

            for sign in &[1.0_f64, -1.0] {
                let pole = [sign * normal[0], sign * normal[1], sign * normal[2]];
                if points.iter().all(|p| dot(&pole, p) >= -1e-12) {
                    return true;
                }
            }
        }
    }

    false
}

fn dot(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn main() {
    let mut rng = rand::rng();
    let num_trials: u64 = 10_000_000;
    let friend = [0.0_f64, 0.0, 1.0]; // friend at north pole

    let epsilons: Vec<f64> = vec![0.001, 0.005, 0.01, 0.02];

    let mut count_a: u64 = 0;
    let mut count_b0: u64 = 0;
    let mut count_b_eps: Vec<u64> = vec![0; epsilons.len()];

    for trial in 0..num_trials {
        if trial % 1_000_000 == 0 && trial > 0 {
            eprintln!("Progress: {:.0}%", 100.0 * trial as f64 / num_trials as f64);
        }

        let planets: Vec<[f64; 3]> = (0..6).map(|_| random_point_on_sphere(&mut rng)).collect();

        let heights: Vec<f64> = planets.iter().map(|p| dot(&friend, p)).collect();
        let min_h = heights.iter().cloned().fold(f64::INFINITY, f64::min);

        let a = in_some_hemisphere(&planets);

        if a {
            count_a += 1;
            if min_h > 0.0 {
                count_b0 += 1;
            }
            for (i, &eps) in epsilons.iter().enumerate() {
                if min_h > -eps {
                    count_b_eps[i] += 1;
                }
            }
        }
    }

    println!("=== Monte Carlo Results ({} trials) ===\n", num_trials);

    let p_a = count_a as f64 / num_trials as f64;
    println!("P(A) = {:.6}  (theory: 0.500000)", p_a);

    let alpha_est = count_b0 as f64 / count_a as f64;
    println!(
        "α = P(friend sees all | A) = {:.6}  (theory: 1/32 = {:.6})",
        alpha_est,
        1.0 / 32.0
    );

    println!("\n--- Beta estimation ---");
    println!("Theory: β = 3/16 = {:.6}\n", 3.0 / 16.0);
    println!(
        "{:<10} {:<18} {:<18} {:<18}",
        "ε", "P(B(ε)|A)", "β_est=(P-α)/ε", "theory β"
    );
    for (i, &eps) in epsilons.iter().enumerate() {
        let p_b_eps = count_b_eps[i] as f64 / count_a as f64;
        let beta_est = (p_b_eps - alpha_est) / eps;
        println!(
            "{:<10.4} {:<18.6} {:<18.6} {:<18.6}",
            eps, p_b_eps, beta_est, 3.0 / 16.0
        );
    }

    println!("\n--- Exact answers ---");
    println!("α = 1/32");
    println!("β = 3/16");
}
