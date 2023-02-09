use num_traits::{FromPrimitive, NumOps, Num, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix<const N: usize, T: Sized + Copy + FromPrimitive + Num + NumOps + Zero> {
    pub vdims: [T; N],
}

pub trait PartialRootable<T: Sized + Copy + FromPrimitive + Num + NumOps + Zero> { fn partial_root(self, root: T) -> Self; }
pub trait ClampableMatrix<T: Sized + Copy + FromPrimitive + Num + NumOps + Zero> { fn mx_clamp(self, min: T, max: T) -> Self; }

#[macro_export]
macro_rules! impl_matrix_for {
    ($N:expr, $T:ty) => {
        impl Matrix<$N, $T> {
            pub fn max() -> Self { Self::from(&[<$T>::MAX; $N]) }
            pub fn min() -> Self { Self::from(&[<$T>::MIN; $N]) }
            pub fn non_zero(&self, repl: $T) -> Self {
                let z: $T = num_traits::zero();
                let mut o = *self;
                for (idx, place) in o.vdims.iter_mut().enumerate() {
                    if self.vdims[idx] == z { *place = repl; }
                };
                return Self::from(o);
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

        impl $crate::matrix::PartialRootable<f64> for Matrix<$N, $T> {
            fn partial_root(self, root: f64) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    let vle: f64 = self.vdims[idx] as f64;
                    *place = $crate::math::nth_root(vle, root).unwrap_or(0.0) as $T;
                };
                return Self::from(&o);
            }
        }

        impl $crate::matrix::PartialRootable<Matrix<$N, $T>> for Matrix<$N, $T> {
            fn partial_root(self, root: Matrix<$N, $T>) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    let vle: f64 = self.vdims[idx] as f64;
                    *place = $crate::math::nth_root(vle, root.vdims[idx] as f64).unwrap_or(0.0) as $T;
                };
                return Self::from(&o);
            }
        }
        
        impl $crate::matrix::ClampableMatrix<Matrix<$N, $T>> for Matrix<$N, $T> {
            fn mx_clamp(self, min: Matrix<$N, $T>, max: Matrix<$N, $T>) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    *place = self.vdims[idx].clamp(min.vdims[idx], max.vdims[idx]);
                };
                return Self::from(&o);
            }
        }

        impl $crate::matrix::ClampableMatrix<$T> for Matrix<$N, $T> {
            fn mx_clamp(self, min: $T, max: $T) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    *place = self.vdims[idx].clamp(max, min);
                };
                return Self::from(&o);
            }
        }

        impl num_traits::Zero for Matrix<$N, $T> {
            fn zero() -> Self { Self::from(&[num_traits::zero(); $N]) }
            fn is_zero(&self) -> bool {
                let z: $T = num_traits::zero();
                return self.vdims.iter().all( | x: &$T | { *x == z } );
            }
        }

        impl num_traits::One  for Matrix<$N, $T> {
            fn one() -> Self { Self::from(&[num_traits::one(); $N]) }
            fn is_one(&self) -> bool {
                let o: $T = num_traits::one();
                return self.vdims.iter().all( | x: &$T | { *x == o } );

            }
        }

        impl num_traits::Num for Matrix<$N, $T> {
            type FromStrRadixErr = &'static str;
            fn from_str_radix(_: &str, _: u32) -> Result<Self, Self::FromStrRadixErr> { unimplemented!() }
        }

        impl Default for Matrix<$N, $T> { fn default() -> Self { num_traits::zero() } }

        impl From<&[$T; $N]> for Matrix<$N, $T> { fn from(vdims: &[$T; $N]) -> Self { Self { vdims: *vdims } } }
        impl From<Matrix<$N, $T>> for [$T; $N] { fn from(hi: Matrix<$N, $T>) -> Self { hi.vdims } }
        impl From<$T> for Matrix<$N, $T> { fn from(n: $T) -> Self { Self::from(&[n; $N]) } }

        impl num_traits::FromPrimitive for Matrix<$N, $T> {
            fn from_u64(_: u64) -> Option<Self> { None } // thanks rust std for not letting me touch primitives
            fn from_i64(_: i64) -> Option<Self> { None } // thanks rust std for not letting me touch primitives
        }
    }
}

#[macro_export]
macro_rules! impl_matrix_aritops_for {
    ($N:expr, $T:ty) => {
        impl std::ops::Add for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn add(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] + rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Div for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn div(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] / rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Mul for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn mul(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] * rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Rem for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn rem(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] % rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Sub for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn sub(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] - rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        
        impl std::ops::AddAssign for Matrix<$N, $T> {
            fn add_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] += rhs.vdims[idx];
                }
            }
        }
        impl std::ops::DivAssign for Matrix<$N, $T> {
            fn div_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] /= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::MulAssign for Matrix<$N, $T> {
            fn mul_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] *= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::RemAssign for Matrix<$N, $T> {
            fn rem_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] %= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::SubAssign for Matrix<$N, $T> {
            fn sub_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] -= rhs.vdims[idx];
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_matrix_aritops_signed_for {
    ($N:expr, $T:ty) => {
        impl_matrix_aritops_for!($N, $T);
        impl std::ops::Neg for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn neg(self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = -self.vdims[idx];
                };
                return Self::from(&o);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_matrix_bitops_for {
    ($N:expr, $T:ty) => {
        impl std::ops::BitAnd for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn bitand(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] & rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::BitOr for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn bitor(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] | rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::BitXor for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn bitxor(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] ^ rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Shl for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn shl(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] << rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Shr for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn shr(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] >> rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Not for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn not(self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = !self.vdims[idx];
                };
                return Self::from(&o);
            }
        }
        
        impl std::ops::BitAndAssign for Matrix<$N, $T> {
            fn bitand_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] &= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::BitOrAssign for Matrix<$N, $T> {
            fn bitor_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] |= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::BitXorAssign for Matrix<$N, $T> {
            fn bitxor_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] ^= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::ShlAssign for Matrix<$N, $T> {
            fn shl_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] <<= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::ShrAssign for Matrix<$N, $T> {
            fn shr_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] >>= rhs.vdims[idx];
                }
            }
        }
    }
}