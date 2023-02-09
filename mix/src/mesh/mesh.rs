mod point;
#[path = "../libs/math.rs"] mod math;

#[cfg(test)] mod test;

use crate::point::Point;
use rand::{rngs::ThreadRng, Rng};

type  T   = i8;
const SIZE: usize = 3;

impl_point_for!(SIZE, T);
impl_point_aritops_for!(SIZE, T);
impl_point_bitops_for!(SIZE, T);

fn main() {
    let mut v: Vec<Point<SIZE, T>> = vec![Point::<SIZE, T>::zero(); 2];
    let mut rng: ThreadRng = rand::thread_rng();

    for _ in 0..rng.gen::<u8>() {
        v.push(Point::<SIZE, T>::from(&rng.gen()));
    };

    for vi in v.iter() {
        println!("{:?}", vi);
    };

    for idx in 0..v.len() {
        let ptx1: Point<SIZE, T> = v[idx];
        let ptx2: Point<SIZE, T> = v[idx.saturating_sub(1)];
        let euclidean_distance: f64 = ptx1.euclidean_distance_to(ptx2);
        let point_distance: [T; SIZE] = ptx1.distances_to(ptx2);
        println!("distance moved from {ptx1:?} to {ptx2:?}:\n\t- {euclidean_distance}\n\t- {point_distance:?}");
    };
}
