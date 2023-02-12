// use crate::{
//     impl_matrix_for,
//     impl_matrix_aritops_for,
//     impl_matrix_aritops_signed_for,
//     matrix::{
//         // Matrix,
//         ClampableMatrix,
//         PartialRootable,
//     }
// };

// use rand::{
//     rngs::ThreadRng,
//     Rng
// };

// use num_traits::{
//     One,
//     Zero
// };

// const SIZE: usize = 2;

// macro_rules! test_matrix {
//     ( $T:ty ) => {
//         if true {
//             let mut v:   Vec<$T>   = vec![<$T>::one(); 2];
//             let mut rng: ThreadRng = rand::thread_rng();
            
//             for _ in 0..rng.gen::<u8>() {
//                 v.push(<$T>::from(&rng.gen()));
//             }
        
//             for idx in 0..v.len() {
//                 let ptx1: $T = v[idx];
//                 let ptx2: $T = v[idx.saturating_sub(1)];
//                 ptx1.euclidean_distance_to(ptx2);
//                 ptx1.distances_to(ptx2);
//             };
//         };
//     };
// }

// macro_rules! test_aritm_matrix {
//     ( $T:ty ) => {
//         if true {
//             let mut v:          Vec<$T>   = vec![<$T>::one(); 2];
//             let mut rng:        ThreadRng = rand::thread_rng();
            
//             let     zero:       $T        = <$T>::zero();
//             let     ptx_mx_sqr: $T        = <$T>::max().partial_root(2.0f64);

//             for _ in 0..rng.gen::<u8>() {
//                 v.push(<$T>::from(&rng.gen()));
//             }

//             // let tn = std::any::type_name::<$T>();
//             // println!("\n\tlet v: Vec<{tn}> = {v:#?}");
        
//             for idx in 0..v.len() {
//                 let ptx1: $T = v[idx];
//                 let ptx2: $T = v[idx.saturating_sub(1)];
                
//                 let _ptx_add: $T = <$T>::min() + ptx1;
//                 let _ptx_sub: $T = <$T>::max() - ptx1;
//                 let _ptx_mul: $T = ptx1.mx_clamp(zero, ptx_mx_sqr) * ptx2.mx_clamp(zero, ptx_mx_sqr);
//                 let _ptx_div: $T = ptx1 / ptx2.non_zero(num_traits::one());
//             };
//         };
//     };
// }

// macro_rules! test_aritm_signed_matrix {
//     ( $T:ty ) => {
//         if true {
//             let mut v = vec![<$T>::zero(); 2];
//             let mut rng: ThreadRng = rand::thread_rng();
            
//             let ptxt:       $T = <$T>::one() + <$T>::one();
//             let ptx_mx_sqr: $T = <$T>::max().partial_root(2.0f64);

//             for _ in 0..rng.gen::<u8>() {
//                 v.push(<$T>::from(&rng.gen()));
//             }

//             // let tn = std::any::type_name::<$T>();
//             // println!("\n\tlet v: Vec<{tn}> = {v:#?}");
        
//             for idx in 0..v.len() {
//                 // both parts are divided by 2 to prevent overflow
//                 let ptx1:    $T =  v[idx] / ptxt;
//                 let ptx2:    $T =  v[idx.saturating_sub(1)] / ptxt;

//                 let _ptx_add: $T =  ptx1 + ptx2;
//                 let _ptx_sub: $T =  ptx1 - ptx2;
//                 let _ptx_mul: $T =  ptx1.mx_clamp(-ptx_mx_sqr, ptx_mx_sqr) * ptx2.mx_clamp(-ptx_mx_sqr, ptx_mx_sqr);
//                 let _ptx_div: $T =  ptx1 / ptx2.non_zero(num_traits::one());
//                 let _ptx_neg: $T = -(ptx1.non_zero(num_traits::one()));
//             };
//         };
//     };
// }

// // Are these impl bleeding into the rest of the crate???
// // BASE
// impl_matrix_for!(SIZE, u8);
// impl_matrix_for!(SIZE, u16);
// impl_matrix_for!(SIZE, u32);
// impl_matrix_for!(SIZE, u64);
// impl_matrix_for!(SIZE, usize);
// impl_matrix_for!(SIZE, u128);

// impl_matrix_for!(SIZE, i8);
// impl_matrix_for!(SIZE, i16);
// impl_matrix_for!(SIZE, i32);
// impl_matrix_for!(SIZE, i64);
// impl_matrix_for!(SIZE, isize);
// impl_matrix_for!(SIZE, i128);

// impl_matrix_for!(SIZE, f32);
// impl_matrix_for!(SIZE, f64);

// // ARITM
// impl_matrix_aritops_for!(SIZE, u8);
// impl_matrix_aritops_for!(SIZE, u16);
// impl_matrix_aritops_for!(SIZE, u32);
// impl_matrix_aritops_for!(SIZE, u64);
// impl_matrix_aritops_for!(SIZE, usize);
// impl_matrix_aritops_for!(SIZE, u128);

// impl_matrix_aritops_signed_for!(SIZE, i8);
// impl_matrix_aritops_signed_for!(SIZE, i16);
// impl_matrix_aritops_signed_for!(SIZE, i32);
// impl_matrix_aritops_signed_for!(SIZE, i64);
// impl_matrix_aritops_signed_for!(SIZE, isize);
// impl_matrix_aritops_signed_for!(SIZE, i128);

// impl_matrix_aritops_signed_for!(SIZE, f32);
// impl_matrix_aritops_signed_for!(SIZE, f64);

// #[test]
// fn test_signed_i8() {
//     test_matrix!(Matrix<SIZE, i8>);
// }

// #[test]
// fn test_signed_i16() {
//     test_matrix!(Matrix<SIZE, i16>);
// }

// #[test]
// fn test_signed_i32() {
//     test_matrix!(Matrix<SIZE, i32>);
// }

// #[test]
// fn test_signed_i64() {
//     test_matrix!(Matrix<SIZE, i64>);
// }

// #[test]
// fn test_signed_isize() {
//     test_matrix!(Matrix<SIZE, isize>);
// }

// #[test]
// fn test_signed_i128() {
//     test_matrix!(Matrix<SIZE, i128>);
// }


// #[test]
// fn test_unsigned_u8() {
//     test_matrix!(Matrix<SIZE, u8>);
// }

// #[test]
// fn test_unsigned_u16() {
//     test_matrix!(Matrix<SIZE, u16>);
// }

// #[test]
// fn test_unsigned_u32() {
//     test_matrix!(Matrix<SIZE, u32>);
// }

// #[test]
// fn test_unsigned_u64() {
//     test_matrix!(Matrix<SIZE, u64>);
// }

// #[test]
// fn test_unsigned_usize() {
//     test_matrix!(Matrix<SIZE, usize>);
// }

// #[test]
// fn test_unsigned_u128() {
//     test_matrix!(Matrix<SIZE, u128>);
// }


// #[test]
// fn test_floats_f32() {
//     test_matrix!(Matrix<SIZE, f32>);
// }

// #[test]
// fn test_floats_f64() {
//     test_matrix!(Matrix<SIZE, f64>);
// }


// #[test] fn test_aritm_signed_i8() {
//     test_aritm_signed_matrix!(Matrix<SIZE, i8>);
// }
// #[test] fn test_aritm_signed_i16() {
//     test_aritm_signed_matrix!(Matrix<SIZE, i16>);
// }
// #[test] fn test_aritm_signed_i32() {
//     test_aritm_signed_matrix!(Matrix<SIZE, i32>);
// }
// #[test] fn test_aritm_signed_i64() {
//     test_aritm_signed_matrix!(Matrix<SIZE, i64>);
// }
// #[test] fn test_aritm_signed_isize() {
//     test_aritm_signed_matrix!(Matrix<SIZE, isize>);
// }
// #[test] fn test_aritm_signed_i128() {
//     test_aritm_signed_matrix!(Matrix<SIZE, i128>);
// }


// #[test] fn test_aritm_unsigned_u8() {
//     test_aritm_matrix!(Matrix<SIZE, u8>);
// }
// #[test] fn test_aritm_unsigned_u16() {
//     test_aritm_matrix!(Matrix<SIZE, u16>);
// }
// #[test] fn test_aritm_unsigned_u32() {
//     test_aritm_matrix!(Matrix<SIZE, u32>);
// }
// #[test] fn test_aritm_unsigned_u64() {
//     test_aritm_matrix!(Matrix<SIZE, u64>);
// }
// #[test] fn test_aritm_unsigned_usize() {
//     test_aritm_matrix!(Matrix<SIZE, usize>);
// }
// #[test] fn test_aritm_unsigned_u128() {
//     test_aritm_matrix!(Matrix<SIZE, u128>);
// }


// #[test]
// fn test_aritm_floats_f32() {
//     test_aritm_signed_matrix!(Matrix<SIZE, f32>);
// }

// #[test]
// fn test_aritm_floats_f64() {
//     test_aritm_signed_matrix!(Matrix<SIZE, f64>);
// }

