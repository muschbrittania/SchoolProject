pub fn compute_pi(n: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let points_inside_circle = (0..n).filter(|_| rng.gen_bool(0.5)).count();
    let pi = 4.0 * points_inside_circle as f64 / n as f64;
    return pi
}
