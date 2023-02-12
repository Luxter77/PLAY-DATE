use std::{ops::{ 
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
}, fmt::{Display, Debug, Formatter, Result}};

use num_traits::{Signed, Num, Zero, One};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SScalar<S: Signed>(pub S);

// BIT OPS
//  Normal
impl<S: Signed + BitAnd<Output = S>> BitAnd for SScalar<S> { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0  & rhs.0) } }
impl<S: Signed + BitOr <Output = S>>  BitOr for SScalar<S> { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.0  | rhs.0) } }
impl<S: Signed + BitXor<Output = S>> BitXor for SScalar<S> { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0  ^ rhs.0) } } 
impl<S: Signed + Shl   <Output = S>>    Shl for SScalar<S> { type Output = Self; fn shl   (self, rhs: Self) -> Self::Output { Self(self.0 << rhs.0) } }
impl<S: Signed + Shr   <Output = S>>    Shr for SScalar<S> { type Output = Self; fn shr   (self, rhs: Self) -> Self::Output { Self(self.0 >> rhs.0) } }
impl<S: Signed + Not   <Output = S>>    Not for SScalar<S> { type Output = Self; fn not   (self) -> Self::Output { Self(!self.0) } }

//  Assign
impl<S: Signed + BitAndAssign> BitAndAssign for SScalar<S> { fn bitand_assign(&mut self, rhs: Self) { self.0 &=  rhs.0 } }
impl<S: Signed + BitOrAssign>   BitOrAssign for SScalar<S> { fn bitor_assign (&mut self, rhs: Self) { self.0 |=  rhs.0 } }
impl<S: Signed + BitXorAssign> BitXorAssign for SScalar<S> { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^=  rhs.0 } }
impl<S: Signed + ShlAssign>       ShlAssign for SScalar<S> { fn shl_assign   (&mut self, rhs: Self) { self.0 <<= rhs.0 } }
impl<S: Signed + ShrAssign>       ShrAssign for SScalar<S> { fn shr_assign   (&mut self, rhs: Self) { self.0 >>= rhs.0 } }

// Math Operations
//  Normal
impl<S: Signed + Add> Add for SScalar<S> { type Output = Self; fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) } }
impl<S: Signed + Div> Div for SScalar<S> { type Output = Self; fn div(self, rhs: Self) -> Self::Output { Self(self.0 / rhs.0) } }
impl<S: Signed + Mul> Mul for SScalar<S> { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { Self(self.0 * rhs.0) } }
impl<S: Signed + Rem> Rem for SScalar<S> { type Output = Self; fn rem(self, rhs: Self) -> Self::Output { Self(self.0 % rhs.0) } }
impl<S: Signed + Sub> Sub for SScalar<S> { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0) } }
impl<S: Signed + Neg> Neg for SScalar<S> { type Output = Self; fn neg(self)            -> Self::Output { Self(-self.0) } }

//  Assign
impl<S: Signed + AddAssign> AddAssign for SScalar<S> { fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0 } }
impl<S: Signed + DivAssign> DivAssign for SScalar<S> { fn div_assign(&mut self, rhs: Self) { self.0 /= rhs.0 } }
impl<S: Signed + MulAssign> MulAssign for SScalar<S> { fn mul_assign(&mut self, rhs: Self) { self.0 *= rhs.0 } }
impl<S: Signed + RemAssign> RemAssign for SScalar<S> { fn rem_assign(&mut self, rhs: Self) { self.0 %= rhs.0 } }
impl<S: Signed + SubAssign> SubAssign for SScalar<S> { fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0 } }

impl<S: Signed + Zero> Zero for SScalar<S> {
    fn set_zero(&mut self) { *self = Self::zero(); }
    fn is_zero(&self) -> bool where Self: PartialEq { *self == Self::zero() }
    fn zero() -> Self { Self(S::zero()) }
}

impl<S: Signed + One> One for SScalar<S> {
    fn set_one(&mut self) { *self = Self::one(); }
    fn is_one(&self) -> bool where Self: PartialEq { *self == Self::one() }
    fn one() -> Self { Self(S::one()) }
}

impl<S: Signed + Num> Num for SScalar<S> {
    type FromStrRadixErr = <S as Num>::FromStrRadixErr;
    fn from_str_radix(str: &str, radix: u32) -> std::result::Result<Self, Self::FromStrRadixErr> {
        match S::from_str_radix(str, radix) {
            Ok(f) => Ok(Self(f)),
            Err(e) => Err(e),
        }
    }
}

impl<S: Signed> Signed for SScalar<S> {
    fn abs(&self) -> Self { Self(self.0.abs()) }
    fn abs_sub(&self, other: &Self) -> Self { Self(self.0.abs_sub(&other.0)) }
    fn signum(&self) -> Self { Self(self.0.signum()) }
    fn is_positive(&self) -> bool { self.0.is_positive() }
    fn is_negative(&self) -> bool { self.0.is_negative() }
}

impl<S: Signed + Debug> Display for SScalar<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Sc<{t}>({v:#?})", t=std::any::type_name::<S>(), v=self.0)
    }
}