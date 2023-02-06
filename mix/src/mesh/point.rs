use num_traits::{FromPrimitive, NumOps, Unsigned, Signed, Float};

#[derive(Debug, Clone, Copy)]
pub struct FPoint<const N: usize, T: Sized + Copy + FromPrimitive + Float + NumOps> {
    pub vdims: [T; N],
}

#[derive(Debug, Clone, Copy)]
pub struct UPoint<const N: usize, T: Sized + Copy + FromPrimitive + Unsigned + NumOps> {
    pub vdims: [T; N],
}
#[derive(Debug, Clone, Copy)]
pub struct IPoint<const N: usize, T: Sized + Copy + FromPrimitive + Signed + NumOps> {
    pub vdims: [T; N],
}

#[macro_export]
macro_rules! impl_point_for {
    ($P:tt, $N:expr, $T:ty) => {
        impl $P<$N, $T> {
            #[allow(unused)] pub fn from_slice(vdims: &[$T; $N]) -> Self { 
                Self {
                    vdims: *vdims,
                }
            }
            #[allow(unused)] pub fn zero() -> Self {
                Self {
                    vdims: [num_traits::zero(); $N],
                }
            }
            #[allow(unused)] pub fn euclidean_distance_to(self, rhs: Self) -> f64 {
                let mut o: f64 = 0.0;
                for idx in 0..$N {
                    let (ptx1, ptx2): (f64, f64) = (self.vdims[idx] as f64, rhs.vdims[idx] as f64);
                    o += if ptx1 >= ptx2 { (ptx1 - ptx2) } else { (ptx2 - ptx1) }.powi(2);
                };
                return o.sqrt();
            }
            #[allow(unused)] pub fn distances_to(self, rhs: Self) -> [$T; $N] {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    let (ptx1, ptx2): (f64, f64) = (self.vdims[idx] as f64, rhs.vdims[idx] as f64);
                    *place = if ptx1 >= ptx2 { (ptx1 - ptx2) } else { (ptx2 - ptx1) }.powi(2).sqrt() as $T;
                };
                return o;
            }
        }

        impl Default for $P<$N, $T> {
            fn default() -> Self { Self::zero() }
        }
    }
}
