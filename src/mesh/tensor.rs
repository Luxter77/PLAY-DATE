// use num_traits::{FromPrimitive, NumOps, Num, Zero};

// use crate::matrix::Matrix;

// pub struct Shape {
//     /// Buffer size for any dimention
//     t_size: usize,
//     /// Number of dimentions
//     n_dims: usize,
//     /// of excactly t_size in len
//     dim_sh: Vec<Shape>,
// }

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Tensor<Shape, const N: usize, T: Sized + Copy + FromPrimitive + Num + NumOps + Zero> {
//     pub shape: Shape,
//     pub vdims: Vec<Vec<Matrix<N, T>>>
// }

