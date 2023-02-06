use crate::{point::Point, impl_point_for};
use rand::{rngs::ThreadRng, Rng};

const SIZE: usize = 5;

macro_rules! test_these {
    ( $( $T:ty ),* ) => {
        $(
            {
                let mut v: Vec<Point<SIZE, $T>> = vec![Point::<SIZE, $T>::zero(); 2];
                let mut rng: ThreadRng = rand::thread_rng();
                
                for _ in 0..rng.gen::<u8>() {
                    v.push(Point::<SIZE, $T>::from_slice(&rng.gen()));
                }
            
                for idx in 0..v.len() {
                    let ptx1: Point<SIZE, $T> = v[idx];
                    let ptx2: Point<SIZE, $T> = v[idx.saturating_sub(1)];
                    let euclidean_distance: f64 = ptx1.euclidean_distance_to(ptx2);
                    let point_distance: [$T; SIZE] = ptx1.distances_to(ptx2);
                };
            }
        )*
    };
}

// Are these impl bleeding into the rest of the crate???

impl_point_for!(SIZE, u8);
impl_point_for!(SIZE, u16);
impl_point_for!(SIZE, u32);
impl_point_for!(SIZE, u64);
impl_point_for!(SIZE, usize);
impl_point_for!(SIZE, u128);

impl_point_for!(SIZE, i8);
impl_point_for!(SIZE, i16);
impl_point_for!(SIZE, i32);
impl_point_for!(SIZE, i64);
impl_point_for!(SIZE, isize);
impl_point_for!(SIZE, i128);

impl_point_for!(SIZE, f32);
impl_point_for!(SIZE, f64);


#[test]
fn test_primitives() {
    test_these!(u8, u16, u32, u64, u128);
    test_these!(i8, i16, i32, i64, i128);
    test_these!(f32, f64);
}