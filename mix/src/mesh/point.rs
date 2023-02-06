use num_traits::{FromPrimitive, Num, NumOps};

#[derive(Debug, Clone, Copy)]
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
                    let t: f64 = if self.vdims[idx] >= rhs.vdims[idx] {
                        (self.vdims[idx] - rhs.vdims[idx]) as f64
                    } else {
                        (rhs.vdims[idx] - self.vdims[idx]) as f64
                    };
                    o += t * t;
                };
                return o.sqrt();
            }
            
            pub fn distances_to(self, rhs: Self) -> [$T; $N] {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    let t: f64 = if self.vdims[idx] >= rhs.vdims[idx] {
                        (self.vdims[idx] - rhs.vdims[idx] ) as f64
                    } else {
                        ( rhs.vdims[idx] - self.vdims[idx]) as f64
                    };
                    *place = (t * t).sqrt() as $T;
                };
                return o;
            }
        }

        impl Default for Point<$N, $T> {
            fn default() -> Self { Self::zero() }
        }
    }
}
