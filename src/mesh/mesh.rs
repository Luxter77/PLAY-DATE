mod matrix;
#[path = "../libs/math.rs"] mod math;
#[cfg(test)] mod tests;

pub mod tensor;
pub mod scalars;
fn main() {}
// #[cfg(not(test))] type  T   = i16;
// #[cfg(not(test))] const SIZE: usize = 3;
// #[cfg(not(test))] fn main() {
//     use crate::matrix::Matrix;
//     use rand::{rngs::ThreadRng, Rng};

//     impl_matrix_for!(SIZE, T);
//     impl_matrix_aritops_for!(SIZE, T);
//     impl_matrix_bitops_for!(SIZE, T);

//     let mut v: Vec<Matrix<SIZE, T>> = vec![num_traits::zero(); 2];
//     let mut rng: ThreadRng = rand::thread_rng();

//     // for _ in 0..rng.gen::<u8>() {
//     for _ in 0..rng.gen::<u8>() {
//         v.push(Matrix::<SIZE, T>::from(&rng.gen()));
//     };

//     for vi in v.iter() {
//         println!("{:?}", vi);
//     };

//     for idx in 0..v.len() {
//         let ptx1: Matrix<SIZE, T> = v[idx];
//         let ptx2: Matrix<SIZE, T> = v[idx.saturating_sub(1)];
//         let euclidean_distance: f64 = ptx1.euclidean_distance_to(ptx2);
//         let point_distance: [T; SIZE] = ptx1.distances_to(ptx2);
//         println!("distance moved from {ptx1:?} to {ptx2:?}:\n\t- {euclidean_distance}\n\t- {point_distance:?}");
//     };
// }