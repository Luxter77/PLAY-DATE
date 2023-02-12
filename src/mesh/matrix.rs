use num_traits::{NumOps, Num, Zero};

#[allow(dead_code)]
pub type Vector<const N: usize, Scalar> = [Scalar; N];

pub trait MeshMatrix<const N: usize, T: Sized + Copy + Num + NumOps + Zero> {
    fn max() -> Self;
    fn min() -> Self;
    fn non_zero(&self, repl: T) -> Self;
    fn euclidean_distance_to(self, rhs: Self) -> f64;
    fn distances_to(self, rhs: Self) -> Self;
}

pub trait NumberIntializable {
    fn zero() -> Self;
    fn one () -> Self;
    fn is_zero(&self) -> bool;
    fn is_one (&self) -> bool;
}

pub trait PartialRootable<T: Sized + Copy> {
    fn partial_root(self, root: T) -> Self;
}
pub trait ClampableMatrix<T: Sized + Copy> {
    fn mx_clamp(self, min: T, max: T) -> Self;
}

pub trait MatrixOps: Sized {
        type Output;
        fn add(self, rhs: Self) -> <Self as MatrixOps>::Output;
        fn div(self, rhs: Self) -> <Self as MatrixOps>::Output;
        fn mul(self, rhs: Self) -> <Self as MatrixOps>::Output;
        fn rem(self, rhs: Self) -> <Self as MatrixOps>::Output;
        fn sub(self, rhs: Self) -> <Self as MatrixOps>::Output;
}

pub trait MatrixOpsNeg: Sized + std::ops::Neg {
    type Output;
    fn neg(self) -> <Self as MatrixOpsNeg>::Output;
}

pub trait MatrixOpsAssign: Sized + std::ops::AddAssign + std::ops::DivAssign + std::ops::MulAssign + std::ops::RemAssign + std::ops::SubAssign {
    fn add_assign(&mut self, rhs: Self);
    fn div_assign(&mut self, rhs: Self);
    fn mul_assign(&mut self, rhs: Self);
    fn rem_assign(&mut self, rhs: Self);
    fn sub_assign(&mut self, rhs: Self);
}

#[macro_export]
macro_rules! impl_matrix_for {
    ($N:expr, $T:ty) => {
        impl $crate::matrix::MeshMatrix<$N, $T> for Matrix<$N, $T> {
            fn max() -> Self { Self::from(&[<$T>::MAX; $N]) }
            fn min() -> Self { Self::from(&[<$T>::MIN; $N]) }
            fn non_zero(&self, repl: $T) -> Self {
                let z: $T = num_traits::zero();
                let mut o = *self;
                for (idx, place) in o.iter_mut().enumerate() {
                    if self[idx] == z { *place = repl; }
                };
                return Self::from(o);
            }
            fn euclidean_distance_to(self, rhs: Self) -> f64 {
                let mut o: f64 = 0.0;
                for idx in 0..$N {
                    let (ptx1, ptx2): (f64, f64) = (self[idx] as f64, rhs[idx] as f64);
                    o += if ptx1 >= ptx2 { (ptx1 - ptx2) } else { (ptx2 - ptx1) }.powi(2);
                };
                return o.sqrt();
            }
            fn distances_to(self, rhs: Self) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    let (ptx1, ptx2): (f64, f64) = (self[idx] as f64, rhs[idx] as f64);
                    *place = if ptx1 >= ptx2 { (ptx1 - ptx2) } else { (ptx2 - ptx1) }.powi(2).sqrt() as $T;
                };
                return o;
            }
        }

        impl $crate::matrix::PartialRootable<f64> for Matrix<$N, $T> {
            fn partial_root(self, root: f64) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    let vle: f64 = self[idx] as f64;
                    *place = $crate::math::nth_root(vle, root).unwrap_or(0.0) as $T;
                };
                return Self::from(&o);
            }
        }

        impl $crate::matrix::PartialRootable<Self> for Matrix<$N, $T> {
            fn partial_root(self, root: Matrix<$N, $T>) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    let vle: f64 = self[idx] as f64;
                    *place = $crate::math::nth_root(vle, root[idx] as f64).unwrap_or(0.0) as $T;
                };
                return Self::from(&o);
            }
        }
        
        impl $crate::matrix::ClampableMatrix<Self> for Matrix<$N, $T> {
            fn mx_clamp(self, min: Matrix<$N, $T>, max: Matrix<$N, $T>) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    *place = self[idx].clamp(min[idx], max[idx]);
                };
                return Self::from(&o);
            }
        }

        impl $crate::matrix::ClampableMatrix<$T> for Matrix<$N, $T> {
            fn mx_clamp(self, min: $T, max: $T) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    *place = self[idx].clamp(max, min);
                };
                return Self::from(&o);
            }
        }

        impl $crate::matrix::NumberIntializable for Matrix<$N, $T> {
            fn zero() -> Self { &[num_traits::zero(); $N] }
            fn one()  -> Self { &[num_traits::one() ; $N] }
            fn is_zero(&self) -> bool {
                let z: $T = num_traits::zero();
                return self.iter().all( | x: &$T | { *x == z } );
            }
            fn is_one(&self) -> bool {
                let o: $T = num_traits::one();
                return self.iter().all( | x: &$T | { *x == o } );
            }
        }
    }
}

#[macro_export]
macro_rules! impl_matrix_aritops_for {
    ($N:expr, $T:ty) => {
        impl $crate::matrix::MatrixOps for Matrix<$N, $T> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self[idx] + rhs[idx];
                }
                return Self::from(&o);
            }
            fn div(self, rhs: Self) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self[idx] / rhs[idx];
                }
                return Self::from(&o);
            }
            fn mul(self, rhs: Self) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self[idx] * rhs[idx];
                }
                return Self::from(&o);
            }
            fn rem(self, rhs: Self) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self[idx] % rhs[idx];
                }
                return Self::from(&o);
            }
            fn sub(self, rhs: Self) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self[idx] - rhs[idx];
                }
                return Self::from(&o);
            }
        }
        
        impl $crate::matrix::MatrixOpsAssign for Matrix<$N, $T> {
            fn add_assign(&mut self, rhs: Self) {
                for idx in 0..self.len() {
                    self[idx] += rhs[idx];
                }
            }
            fn div_assign(&mut self, rhs: Self) {
                for idx in 0..self.len() {
                    self[idx] /= rhs[idx];
                }
            }
            fn mul_assign(&mut self, rhs: Self) {
                for idx in 0..self.len() {
                    self[idx] *= rhs[idx];
                }
            }
            fn rem_assign(&mut self, rhs: Self) {
                for idx in 0..self.len() {
                    self[idx] %= rhs[idx];
                }
            }
            fn sub_assign(&mut self, rhs: Self) {
                for idx in 0..self.len() {
                    self[idx] -= rhs[idx];
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_matrix_aritops_signed_for {
    ($N:expr, $T:ty) => {
        impl_matrix_aritops_for!($N, $T);
        // impl $crate::matrix::MatrixOpsNeg: std::ops::Neg for Matrix<$N, $T> {
        //     type Output = Matrix<$N, $T>;
        //     fn neg(self) -> Self::Output {
        //         let mut o: [$T; $N] = [num_traits::zero(); $N];
        //         for (idx, part) in o.iter_mut().enumerate() {
        //             *part = -self[idx];
        //         };
        //         return Self::from(&o);
        //     }
        // }
    };
}

#[macro_export]
macro_rules! impl_matrix_bitops_for {
    ($N:expr, $T:ty) => {
        impl std::ops::BitAnd for Scalar<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn bitand(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self[idx] & rhs[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::BitOr for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn bitor(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self[idx] | rhs[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::BitXor for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn bitxor(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self[idx] ^ rhs[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Shl for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn shl(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self[idx] << rhs[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Shr for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn shr(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self[idx] >> rhs[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Not for Matrix<$N, $T> {
            type Output = Matrix<$N, $T>;
            fn not(self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = !self[idx];
                };
                return Self::from(&o);
            }
        }
        
        impl std::ops::BitAndAssign for Matrix<$N, $T> {
            fn bitand_assign(&mut self, rhs: Self) {
                for idx in 0..self.len() {
                    self[idx] &= rhs[idx];
                }
            }
        }
        impl std::ops::BitOrAssign for Matrix<$N, $T> {
            fn bitor_assign(&mut self, rhs: Self) {
                for idx in 0..self.len() {
                    self[idx] |= rhs[idx];
                }
            }
        }
        impl std::ops::BitXorAssign for Matrix<$N, $T> {
            fn bitxor_assign(&mut self, rhs: Self) {
                for idx in 0..self.len() {
                    self[idx] ^= rhs[idx];
                }
            }
        }
        impl std::ops::ShlAssign for Matrix<$N, $T> {
            fn shl_assign(&mut self, rhs: Self) {
                for idx in 0..self.len() {
                    self[idx] <<= rhs[idx];
                }
            }
        }
        impl std::ops::ShrAssign for Matrix<$N, $T> {
            fn shr_assign(&mut self, rhs: Self) {
                for idx in 0..self.len() {
                    self[idx] >>= rhs[idx];
                }
            }
        }
    }
}