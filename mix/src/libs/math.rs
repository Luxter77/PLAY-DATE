/// https://rosettacode.org/wiki/Nth_root#Rust
/// 20210212 Rust programming solution
pub fn nth_root(n: f64, a: f64) -> f64 {
    let p: f64 = 1e-9_f64;
    let mut x0: f64 = a / n;
    let mut x1: f64;
    assert!(n >= 0.0, "n must be positive!");
    loop {
        x1 = ((n - 1.0) * x0 + a / f64::powf(x0, n - 1.0)) / n;
        if (x1 - x0).abs() < (x0 * p).abs() { return x1; };
        x0 = x1;
    }
}
