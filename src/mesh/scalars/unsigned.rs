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
    // Neg,
}, fmt::{Display, Debug, Formatter, Result}};

use num_traits::{Unsigned, Zero, One, Num};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UScalar<U: Unsigned>(pub U);

// BIT OPS
//  Normal
impl<U: Unsigned + BitAnd<Output = U>> BitAnd for UScalar<U> { type Output = Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0  & rhs.0) } }
impl<U: Unsigned + BitOr <Output = U>>  BitOr for UScalar<U> { type Output = Self; fn bitor (self, rhs: Self) -> Self::Output { Self(self.0  | rhs.0) } }
impl<U: Unsigned + BitXor<Output = U>> BitXor for UScalar<U> { type Output = Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0  ^ rhs.0) } } 
impl<U: Unsigned + Shl   <Output = U>>    Shl for UScalar<U> { type Output = Self; fn shl   (self, rhs: Self) -> Self::Output { Self(self.0 << rhs.0) } }
impl<U: Unsigned + Shr   <Output = U>>    Shr for UScalar<U> { type Output = Self; fn shr   (self, rhs: Self) -> Self::Output { Self(self.0 >> rhs.0) } }
impl<U: Unsigned + Not   <Output = U>>    Not for UScalar<U> { type Output = Self; fn not   (self) -> Self::Output { Self(!self.0) } }

//  Assign
impl<U: Unsigned + BitAndAssign> BitAndAssign for UScalar<U> { fn bitand_assign(&mut self, rhs: Self) { self.0 &=  rhs.0 } }
impl<U: Unsigned + BitOrAssign>   BitOrAssign for UScalar<U> { fn bitor_assign (&mut self, rhs: Self) { self.0 |=  rhs.0 } }
impl<U: Unsigned + BitXorAssign> BitXorAssign for UScalar<U> { fn bitxor_assign(&mut self, rhs: Self) { self.0 ^=  rhs.0 } }
impl<U: Unsigned + ShlAssign>       ShlAssign for UScalar<U> { fn shl_assign   (&mut self, rhs: Self) { self.0 <<= rhs.0 } }
impl<U: Unsigned + ShrAssign>       ShrAssign for UScalar<U> { fn shr_assign   (&mut self, rhs: Self) { self.0 >>= rhs.0 } }

// Math Operations
//  Normal
impl<U: Unsigned + Add> Add for UScalar<U> { type Output = Self; fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) } }
impl<U: Unsigned + Div> Div for UScalar<U> { type Output = Self; fn div(self, rhs: Self) -> Self::Output { Self(self.0 / rhs.0) } }
impl<U: Unsigned + Mul> Mul for UScalar<U> { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { Self(self.0 * rhs.0) } }
impl<U: Unsigned + Rem> Rem for UScalar<U> { type Output = Self; fn rem(self, rhs: Self) -> Self::Output { Self(self.0 % rhs.0) } }
impl<U: Unsigned + Sub> Sub for UScalar<U> { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0) } }
// impl<U: Unsigned> Neg for UScalar<U> { type Output = Self; fn neg(self)            -> Self::Output { Self(-self.0) } }

//  Assign
impl<U: Unsigned + AddAssign> AddAssign for UScalar<U> { fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0 } }
impl<U: Unsigned + DivAssign> DivAssign for UScalar<U> { fn div_assign(&mut self, rhs: Self) { self.0 /= rhs.0 } }
impl<U: Unsigned + MulAssign> MulAssign for UScalar<U> { fn mul_assign(&mut self, rhs: Self) { self.0 *= rhs.0 } }
impl<U: Unsigned + RemAssign> RemAssign for UScalar<U> { fn rem_assign(&mut self, rhs: Self) { self.0 %= rhs.0 } }
impl<U: Unsigned + SubAssign> SubAssign for UScalar<U> { fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0 } }

impl<U: Unsigned + Zero> Zero for UScalar<U> {
    fn set_zero(&mut self) { *self = Self::zero(); }
    fn is_zero(&self) -> bool where Self: PartialEq { *self == Self::zero() }
    fn zero() -> Self { Self(U::zero()) }
}

impl<U: Unsigned + One> One for UScalar<U> {
    fn set_one(&mut self) { *self = Self::one(); }
    fn is_one(&self) -> bool where Self: PartialEq { *self == Self::one() }
    fn one() -> Self { Self(U::one()) }
}

impl<U: Unsigned + Num> Num for UScalar<U> {
    type FromStrRadixErr = <U as Num>::FromStrRadixErr;
    fn from_str_radix(str: &str, radix: u32) -> std::result::Result<Self, Self::FromStrRadixErr> {
        match U::from_str_radix(str, radix) {
            Ok(f) => Ok(Self(f)),
            Err(e) => Err(e),
        }
    }
}

impl<U: Unsigned> Unsigned for UScalar<U> {}

impl<U: Unsigned + std::fmt::Debug> Display for UScalar<U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Sc<{t}>({v:#?})", t=std::any::type_name::<U>(), v=self.0)
    }
}
