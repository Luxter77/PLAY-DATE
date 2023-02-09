/// https://rosettacode.org/wiki/Nth_root#Rust
/// 20210212 Rust programming solution
pub fn nth_root(n: f64, a: f64) -> Option<f64> {
    assert!(n >= 0.0, "n must be positive!");
    let p: f64 = 1e-9_f64;
    let mut x0: f64 = a / n;
    let mut x1: f64;
    loop {
        x1 = ((n - 1.0) * x0 + a / f64::powf(x0, n - 1.0)) / n;
        if (x1 - x0).abs() < (x0 * p).abs() {
            return Option::Some(x1);
        } else if !x1.is_normal() {
            return None
        };
        x0 = x1;
    }
}
