use num_traits::{FromPrimitive, NumOps, Num, Zero};

use crate::matrix::Matrix;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Shape {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tensor<const D: usize, const N: usize, T: Sized + Copy + FromPrimitive + Num + NumOps + Zero> {
    pub shape: Shape,
    pub vdims: Vec<Matrix<N, T>>,
}

