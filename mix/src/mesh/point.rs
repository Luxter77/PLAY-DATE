use num_traits::{FromPrimitive, NumOps, Num, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<const N: usize, T: Sized + Copy + FromPrimitive + Num + NumOps + Zero> {
    pub vdims: [T; N],
}

#[macro_export]
macro_rules! impl_point_for {
    ($N:expr, $T:ty) => {
        impl Point<$N, $T> {
            pub fn zero() -> Self { Self::from(&[num_traits::zero(); $N]) }
            pub fn one()  -> Self { Self::from(&[num_traits::one();  $N]) }
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
            pub fn partial_point_root(self, root: f64) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    let vle: f64 = self.vdims[idx] as f64;
                    *place = $crate::math::nth_root(vle, root).unwrap_or(0.0) as $T;
                };
                return Self::from(&o);
            }
            pub fn partial_root(self, root: Self) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    let vle: f64 = self.vdims[idx] as f64;
                    *place = $crate::math::nth_root(vle, root.vdims[idx] as f64).unwrap_or(0.0) as $T;
                };
                return Self::from(&o);
            }
            pub fn point_clamp(self, min: Self, max: Self) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    *place = self.vdims[idx].clamp(min.vdims[idx], max.vdims[idx]);
                };
                return Self::from(&o);
            }
            pub fn number_clamp(self, min: $T, max: $T) -> Self {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, place) in o.iter_mut().enumerate() {
                    *place = self.vdims[idx].clamp(max, min);
                };
                return Self::from(&o);
            }
        }

        impl Default for Point<$N, $T> {
            fn default() -> Self { Self::zero() }
        }

        impl From<&[$T; $N]> for Point<$N, $T> {
            fn from(vdims: &[$T; $N]) -> Self { Self { vdims: *vdims } }
        }

        impl From<Point<$N, $T>> for [$T; $N] {
            fn from(hi: Point<$N, $T>) -> Self { hi.vdims }
        }
        impl From<$T> for Point<$N, $T> {
            fn from(n: $T) -> Self { Self::from(&[n; $N]) }
        }
    }
}

#[macro_export]
macro_rules! impl_point_aritops_for {
    ($N:expr, $T:ty) => {
        impl std::ops::Add for Point<$N, $T> {
            type Output = Point<$N, $T>;
            fn add(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] + rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Div for Point<$N, $T> {
            type Output = Point<$N, $T>;
            fn div(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] / rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Mul for Point<$N, $T> {
            type Output = Point<$N, $T>;
            fn mul(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] * rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Rem for Point<$N, $T> {
            type Output = Point<$N, $T>;
            fn rem(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] % rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Sub for Point<$N, $T> {
            type Output = Point<$N, $T>;
            fn sub(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] - rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        
        impl std::ops::AddAssign for Point<$N, $T> {
            fn add_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] += rhs.vdims[idx];
                }
            }
        }
        impl std::ops::DivAssign for Point<$N, $T> {
            fn div_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] /= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::MulAssign for Point<$N, $T> {
            fn mul_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] *= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::RemAssign for Point<$N, $T> {
            fn rem_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] %= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::SubAssign for Point<$N, $T> {
            fn sub_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] -= rhs.vdims[idx];
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_point_aritops_signed_for {
    ($N:expr, $T:ty) => {
        impl_point_aritops_for!($N, $T);
        impl std::ops::Neg for Point<$N, $T> {
            type Output = Point<$N, $T>;
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
macro_rules! impl_point_bitops_for {
    ($N:expr, $T:ty) => {
        impl std::ops::BitAnd for Point<$N, $T> {
            type Output = Point<$N, $T>;
            fn bitand(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] & rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::BitOr for Point<$N, $T> {
            type Output = Point<$N, $T>;
            fn bitor(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] | rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::BitXor for Point<$N, $T> {
            type Output = Point<$N, $T>;
            fn bitxor(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] ^ rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Shl for Point<$N, $T> {
            type Output = Point<$N, $T>;
            fn shl(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] << rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Shr for Point<$N, $T> {
            type Output = Point<$N, $T>;
            fn shr(self, rhs: Self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = self.vdims[idx] >> rhs.vdims[idx];
                }
                return Self::from(&o);
            }
        }
        impl std::ops::Not for Point<$N, $T> {
            type Output = Point<$N, $T>;
            fn not(self) -> Self::Output {
                let mut o: [$T; $N] = [num_traits::zero(); $N];
                for (idx, part) in o.iter_mut().enumerate() {
                    *part = !self.vdims[idx];
                };
                return Self::from(&o);
            }
        }
        
        impl std::ops::BitAndAssign for Point<$N, $T> {
            fn bitand_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] &= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::BitOrAssign for Point<$N, $T> {
            fn bitor_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] |= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::BitXorAssign for Point<$N, $T> {
            fn bitxor_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] ^= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::ShlAssign for Point<$N, $T> {
            fn shl_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] <<= rhs.vdims[idx];
                }
            }
        }
        impl std::ops::ShrAssign for Point<$N, $T> {
            fn shr_assign(&mut self, rhs: Self) {
                for idx in 0..self.vdims.len() {
                    self.vdims[idx] >>= rhs.vdims[idx];
                }
            }
        }
    }
}
