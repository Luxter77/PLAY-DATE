use num_traits::{Float, Signed, Unsigned};

mod float;
mod signed;
mod unsigned;

pub use float::FScalar;
pub use signed::SScalar;
pub use unsigned::UScalar;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AutoScalar<F: Float, S: Signed, U: Unsigned> { F(FScalar<F>), S(SScalar<S>), U(UScalar<U>) }

impl<F: Float, S: Signed + Default, U: Unsigned> Default for AutoScalar<F, S, U> {
    fn default() -> Self { Self::S(SScalar::<S>::default()) }
}