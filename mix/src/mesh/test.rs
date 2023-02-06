use crate::{impl_point_for, point::{UPoint, IPoint, FPoint}};
use rand::{rngs::ThreadRng, Rng};

const SIZE: usize = 5;

macro_rules! test_point {
    ( $T:ty ) => {
        if true {
            let mut v = vec![<$T>::zero(); 2];
            let mut rng: ThreadRng = rand::thread_rng();
            
            for _ in 0..rng.gen::<u8>() {
                v.push(<$T>::from_slice(&rng.gen()));
            }
        
            for idx in 0..v.len() {
                let ptx1: $T = v[idx];
                let ptx2: $T = v[idx.saturating_sub(1)];
                ptx1.euclidean_distance_to(ptx2);
                ptx1.distances_to(ptx2);
            };
        };
    };
}

// Are these impl bleeding into the rest of the crate???

impl_point_for!(UPoint, SIZE, u8);
impl_point_for!(UPoint, SIZE, u16);
impl_point_for!(UPoint, SIZE, u32);
impl_point_for!(UPoint, SIZE, u64);
impl_point_for!(UPoint, SIZE, usize);
impl_point_for!(UPoint, SIZE, u128);
impl_point_for!(IPoint, SIZE, i8);
impl_point_for!(IPoint, SIZE, i16);
impl_point_for!(IPoint, SIZE, i32);
impl_point_for!(IPoint, SIZE, i64);
impl_point_for!(IPoint, SIZE, isize);
impl_point_for!(IPoint, SIZE, i128);
impl_point_for!(FPoint, SIZE, f32);
impl_point_for!(FPoint, SIZE, f64);


#[test]
fn test_signed() {
    test_point!(IPoint<SIZE, i8>);
    test_point!(IPoint<SIZE, i16>);
    test_point!(IPoint<SIZE, i32>);
    test_point!(IPoint<SIZE, i64>);
    test_point!(IPoint<SIZE, i128>);
}

#[test]
fn test_unsigned() {
    test_point!(UPoint<SIZE, u8>);
    test_point!(UPoint<SIZE, u16>);
    test_point!(UPoint<SIZE, u32>);
    test_point!(UPoint<SIZE, u64>);
    test_point!(UPoint<SIZE, u128>);
}

#[test]
fn test_floats() {
    test_point!(FPoint<SIZE, f32>);
    test_point!(FPoint<SIZE, f64>);
}

