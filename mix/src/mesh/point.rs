use num_traits::{FromPrimitive, NumOps, Num};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<const N: usize, T: Sized + Copy + FromPrimitive + Num + NumOps> {
    pub vdims: [T; N],
}

#[macro_export]
macro_rules! impl_point_for {
    ($N:expr, $T:ty) => {
        impl Point<$N, $T> {
            pub fn from_slice(vdims: &[$T; $N]) -> Self { 
                Self {
                    vdims: *vdims,
                }
            }
            pub fn zero() -> Self {
                Self {
                    vdims: [num_traits::zero(); $N],
                }
            }
            pub fn euclidean_distance_to(self, rhs: Self) -> f64 {
                let mut o: f64 = 0.0;
                for idx in 0..$N {
                    let (ptx1, ptx2): (f64, f64) = (self.vdims[idx] as f64, rhs.vdims[idx] as f64);
                    o += if ptx1 >= ptx2 { (ptx1 - ptx2) } else { (ptx2 - ptx1) }.powi(2);
                };
                return o.sqrt();
            }
            pub fn distances_to(self, rhs: Self) -> [$T; $N] {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    let (ptx1, ptx2): (f64, f64) = (self.vdims[idx] as f64, rhs.vdims[idx] as f64);
                    *place = if ptx1 >= ptx2 { (ptx1 - ptx2) } else { (ptx2 - ptx1) }.powi(2).sqrt() as $T;
                };
                return o;
            }
        }

        impl Default for Point<$N, $T> {
            fn default() -> Self { Self::zero() }
        }
    }
}
