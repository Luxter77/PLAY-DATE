use num_traits::Float;

/// https://rosettacode.org/wiki/Nth_root#Rust
/// 20210212 Rust programming solution
#[allow(unused)]
pub fn nth_root<F: Float>(a: F, n: F) -> Option<F> {
    assert!(n >= F::zero(), "n must be positive!");
    let p: F = F::from(1e-9f64).expect("Cannot reprecent magic number p in terms of <F>");
    let mut x0: F = a / n;
    let mut x1: F;
    loop {
        x1 = ((n - F::one()) * x0 + a / F::powf(x0, n - F::one())) / n;
        if (x1 - x0).abs() < (x0 * p).abs() {
            return Option::Some(((n - F::one()) * x0 + a / F::powf(x0, n - F::one())) / n);
        } else if !x1.is_normal() { return None };
        x0 = x1;
    }
}

#[cfg(test)] mod tests {
    use num_bigfloat::BigFloat;
    use std::str::FromStr;

    /// WolframAlpha:tm: says root(1.1, 2.2) equals to this number
    const RES_ROOT_1_1__2_2: &'static str = "1.0442749417985312249839423448985727149343419768498446903066";

    /// Prescition breaks down about here for my computer;
    /// So thats the magic number now, I guess
    const DIF_ROOT_1_1__2_2: &'static str = "0.0000000000000003";

    #[test] fn test_nth_root() {
        let r__: f32      = f32::     from_str(RES_ROOT_1_1__2_2).unwrap();
        let rr_: f64      = f64::     from_str(RES_ROOT_1_1__2_2).unwrap();
        let rrr: BigFloat = BigFloat::from_str(RES_ROOT_1_1__2_2).unwrap();
        let rr4: f64      = 0.0;

        let d__: f32      = f32::     from_str(DIF_ROOT_1_1__2_2).unwrap();
        let dd_: f64      = f64::     from_str(DIF_ROOT_1_1__2_2).unwrap();
        let ddd: BigFloat = BigFloat::from_str(DIF_ROOT_1_1__2_2).unwrap();
        let dd4: f64      = 4.0;

        let n__: f32      = super::nth_root(1.1f32,                 2.2f32).unwrap_or_default();
        let nn_: f64      = super::nth_root(1.1f64,                 2.2f64).unwrap_or_default();
        let nnn: BigFloat = super::nth_root(BigFloat::from(1.1f64), BigFloat::from(2.2f64)).unwrap_or_default();
        let nn4: f64      = super::nth_root(4.0f64,                 2.0f64).unwrap_or_default();

        assert!((n__ - r__).abs() < d__, "inequality found in f32      comparison {n__:?} == {r__:?}\n");
        assert!((nn_ - rr_).abs() < dd_, "inequality found in f64      comparison {nn_:?} == {rr_:?}\n");
        assert!((nnn - rrr).abs() < ddd, "inequality found in BigFloat comparison {nnn:?} == {rrr:?}\n");
        assert!((nn4 - rr4).abs() < dd4, "inequality found in BigFloat comparison {nnn:?} == {rrr:?}\n");
    }
}