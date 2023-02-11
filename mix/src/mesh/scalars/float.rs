use std::{
    ops::{ 
        BitAnd, BitAndAssign,
        BitOr,  BitOrAssign,
        BitXor, BitXorAssign,
        Shl,    ShlAssign,
        Shr,    ShrAssign,
        Not,
        Add,    AddAssign,
        Div,    DivAssign,
        Mul,    MulAssign,
        Rem,    RemAssign,
        Sub,    SubAssign,
        Neg,    
    },
    fmt::{
        Display,
        Debug,
        Formatter,
        Result
    }
};

use num_traits::{Float, Num, Zero, One, NumCast, ToPrimitive};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FScalar<F: Float>(pub F);

// BIT OPS
//  Normal
impl<F: Float + BitAnd<Output = F>> BitAnd for FScalar<F> { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0  & rhs.0) } }
impl<F: Float + BitOr <Output = F>>  BitOr for FScalar<F> { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.0  | rhs.0) } }
impl<F: Float + BitXor<Output = F>> BitXor for FScalar<F> { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0  ^ rhs.0) } } 
impl<F: Float + Shl   <Output = F>>    Shl for FScalar<F> { type Output = Self; fn shl   (self, rhs: Self) -> Self::Output { Self(self.0 << rhs.0) } }
impl<F: Float + Shr   <Output = F>>    Shr for FScalar<F> { type Output = Self; fn shr   (self, rhs: Self) -> Self::Output { Self(self.0 >> rhs.0) } }
impl<F: Float + Not   <Output = F>>    Not for FScalar<F> { type Output = Self; fn not   (self) -> Self::Output { Self(!self.0) } }
//  Assign
impl<F: Float + BitAndAssign> BitAndAssign for FScalar<F> { fn bitand_assign(&mut self, rhs: Self) { self.0 &=  rhs.0 } }
impl<F: Float + BitOrAssign>   BitOrAssign for FScalar<F> { fn bitor_assign (&mut self, rhs: Self) { self.0 |=  rhs.0 } }
impl<F: Float + BitXorAssign> BitXorAssign for FScalar<F> { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^=  rhs.0 } }
impl<F: Float + ShlAssign>       ShlAssign for FScalar<F> { fn shl_assign   (&mut self, rhs: Self) { self.0 <<= rhs.0 } }
impl<F: Float + ShrAssign>       ShrAssign for FScalar<F> { fn shr_assign   (&mut self, rhs: Self) { self.0 >>= rhs.0 } }
// Math Operations
//  Normal
impl<F: Float + Add> Add for FScalar<F> { type Output = Self; fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) } }
impl<F: Float + Div> Div for FScalar<F> { type Output = Self; fn div(self, rhs: Self) -> Self::Output { Self(self.0 / rhs.0) } }
impl<F: Float + Mul> Mul for FScalar<F> { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { Self(self.0 * rhs.0) } }
impl<F: Float + Rem> Rem for FScalar<F> { type Output = Self; fn rem(self, rhs: Self) -> Self::Output { Self(self.0 % rhs.0) } }
impl<F: Float + Sub> Sub for FScalar<F> { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0) } }
impl<F: Float + Neg> Neg for FScalar<F> { type Output = Self; fn neg(self)            -> Self::Output { Self(-self.0) } }
//  Assign
impl<F: Float + AddAssign> AddAssign for FScalar<F> { fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0 } }
impl<F: Float + DivAssign> DivAssign for FScalar<F> { fn div_assign(&mut self, rhs: Self) { self.0 /= rhs.0 } }
impl<F: Float + MulAssign> MulAssign for FScalar<F> { fn mul_assign(&mut self, rhs: Self) { self.0 *= rhs.0 } }
impl<F: Float + RemAssign> RemAssign for FScalar<F> { fn rem_assign(&mut self, rhs: Self) { self.0 %= rhs.0 } }
impl<F: Float + SubAssign> SubAssign for FScalar<F> { fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0 } }

impl<F: Float + Zero> Zero for FScalar<F> {
    fn set_zero(&mut self) { *self = Self::zero(); }
    fn is_zero(&self) -> bool where Self: PartialEq { *self == Self::zero() }
    fn zero() -> Self { FScalar(F::zero()) }
}

impl<F: Float + One> One for FScalar<F> {
    fn set_one(&mut self) { *self = Self::one(); }
    fn is_one(&self) -> bool where Self: PartialEq { *self == Self::one() }
    fn one() -> Self { Self(F::one()) }
}

impl<F: Float + Num> Num for FScalar<F> {
    type FromStrRadixErr = <F as Num>::FromStrRadixErr;
    fn from_str_radix(str: &str, radix: u32) -> std::result::Result<Self, Self::FromStrRadixErr> {
        match F::from_str_radix(str, radix) {
            Ok(f) => Ok(Self(f)),
            Err(e) => Err(e),
        }
    }
}

impl<F: Float + ToPrimitive> ToPrimitive for FScalar<F> {
    fn to_isize(&self) -> Option<isize> { self.to_i64().as_ref().and_then(ToPrimitive::to_isize) }
    fn to_i8(&self) -> Option<i8> { self.to_i64().as_ref().and_then(ToPrimitive::to_i8) }
    fn to_i16(&self) -> Option<i16> { self.to_i64().as_ref().and_then(ToPrimitive::to_i16) }
    fn to_i32(&self) -> Option<i32> { self.to_i64().as_ref().and_then(ToPrimitive::to_i32) }
    fn to_i128(&self) -> Option<i128> { self.to_i64().map(From::from) }
    fn to_usize(&self) -> Option<usize> { self.to_u64().as_ref().and_then(ToPrimitive::to_usize) }
    fn to_u8(&self) -> Option<u8> { self.to_u64().as_ref().and_then(ToPrimitive::to_u8) }
    fn to_u16(&self) -> Option<u16> { self.to_u64().as_ref().and_then(ToPrimitive::to_u16) }
    fn to_u32(&self) -> Option<u32> { self.to_u64().as_ref().and_then(ToPrimitive::to_u32) }
    fn to_u128(&self) -> Option<u128> { self.to_u64().map(From::from) }
    fn to_f32(&self) -> Option<f32> { self.to_f64().as_ref().and_then(ToPrimitive::to_f32) }
    fn to_f64(&self) -> Option<f64> {
        match self.to_i64() {
            Some(i) => i.to_f64(),
            None => self.to_u64().as_ref().and_then(ToPrimitive::to_f64),
        }
    }
    fn to_i64(&self) -> Option<i64> { self.0.to_i64() }
    fn to_u64(&self) -> Option<u64> { self.0.to_u64() }
}

impl<F: Float + NumCast> NumCast for FScalar<F> {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        match F::from(n) {
            Some(nn) => Some(Self(nn)),
            None => None,
        }
    }
}

impl<F: Float> Float for FScalar<F> {
    fn epsilon() -> Self { <FScalar<F> as NumCast>::from(F::epsilon()).expect("Unable to cast from F::epsilon()") }
    fn to_degrees(self) -> Self {
        let halfpi = Self::zero().acos();
        let ninety = <FScalar<F> as NumCast>::from(90u8).unwrap();
        self * ninety / halfpi
    }

    fn to_radians(self) -> Self {
        let halfpi = Self::zero().acos();
        let ninety = <FScalar<F> as NumCast>::from(90u8).unwrap();
        self * halfpi / ninety
    }

    fn copysign(self, sign: Self) -> Self { if self.is_sign_negative() == sign.is_sign_negative() { self } else { self.neg() } }
    
    fn nan() -> Self { Self(F::nan()) }
    fn infinity() -> Self { Self(F::infinity()) }
    fn neg_infinity() -> Self { Self(F::neg_infinity()) }
    fn neg_zero() -> Self { Self(F::neg_zero()) }
    fn min_value() -> Self { Self(F::min_value()) }
    fn min_positive_value() -> Self { Self(F::min_positive_value()) }
    fn max_value() -> Self { Self(F::max_value()) }
    fn is_nan(self) -> bool { self.0.is_nan() }
    fn is_infinite(self) -> bool { self.0.is_infinite() }
    fn is_finite(self) -> bool { self.0.is_finite() }
    fn is_normal(self) -> bool { self.0.is_normal() }
    fn classify(self) -> std::num::FpCategory { self.0.classify() }
    fn floor(self) -> Self { Self(self.0.floor()) }
    fn ceil(self) -> Self { Self(self.0.ceil()) }
    fn round(self) -> Self { Self(self.0.round()) }
    fn trunc(self) -> Self { Self(self.0.trunc()) }
    fn fract(self) -> Self { Self(self.0.fract()) }
    fn abs(self) -> Self { Self(self.0.abs()) }
    fn signum(self) -> Self { Self(self.0.signum()) }
    fn is_sign_positive(self) -> bool { self.0.is_sign_positive() }
    fn is_sign_negative(self) -> bool { self.0.is_sign_negative() }
    fn mul_add(self, a: Self, b: Self) -> Self { Self(self.0.mul_add(a.0, b.0)) }
    fn recip(self) -> Self { Self(self.0.recip()) }
    fn powi(self, n: i32) -> Self { Self(self.0.powi(n)) }
    fn powf(self, n: Self) -> Self { Self(self.0.powf(n.0)) }
    fn sqrt(self) -> Self { Self(self.0.sqrt()) }
    fn exp(self) -> Self { Self(self.0.exp()) }
    fn exp2(self) -> Self { Self(self.0.exp2()) }
    fn ln(self) -> Self { Self(self.0.ln()) }
    fn log(self, base: Self) -> Self { Self(self.0.log(base.0)) }
    fn log2(self) -> Self { Self(self.0.log2()) }
    fn log10(self) -> Self { Self(self.0.log10()) }
    fn max(self, other: Self) -> Self { Self(self.0.max(other.0)) }
    fn min(self, other: Self) -> Self { Self(self.0.min(other.0)) }
    fn abs_sub(self, other: Self) -> Self { Self(self.0.abs_sub(other.0)) }
    fn cbrt(self) -> Self { Self(self.0.cbrt()) }
    fn hypot(self, other: Self) -> Self { Self(self.0.hypot(other.0)) }
    fn sin(self) -> Self { Self(self.0.sin()) }
    fn cos(self) -> Self { Self(self.0.cos()) }
    fn tan(self) -> Self { Self(self.0.tan()) }
    fn asin(self) -> Self { Self(self.0.asin()) }
    fn acos(self) -> Self { Self(self.0.acos()) }
    fn atan(self) -> Self { Self(self.0.atan()) }
    fn atan2(self, other: Self) -> Self { Self(self.0.atan2(other.0)) }
    fn sin_cos(self) -> (Self, Self) { let (a, b) = self.0.sin_cos(); return (Self(a), Self(b)) }
    fn exp_m1(self) -> Self { Self(self.0.exp_m1()) }
    fn ln_1p(self) -> Self { Self(self.0.ln_1p()) }
    fn sinh(self) -> Self { Self(self.0.sinh()) }
    fn cosh(self) -> Self { Self(self.0.cosh()) }
    fn tanh(self) -> Self { Self(self.0.tanh()) }
    fn asinh(self) -> Self { Self(self.0.asinh()) }
    fn acosh(self) -> Self { Self(self.0.acosh()) }
    fn atanh(self) -> Self { Self(self.0.atanh()) }
    fn integer_decode(self) -> (u64, i16, i8) { self.0.integer_decode() }
}

impl<F: Float + Debug> Display for FScalar<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Sc<{t}>({v:#?})", t=std::any::type_name::<F>(), v=self.0)
    }
}
