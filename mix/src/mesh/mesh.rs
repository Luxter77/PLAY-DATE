mod point;

#[cfg(test)] mod test;

use crate::point::IPoint;
use rand::{rngs::ThreadRng, Rng};

type  T   = i8;
type  P<const N: usize, T>   = IPoint<N, T>;
const SIZE: usize = 3;

impl_point_for!(P, SIZE, T);

fn main() {
    let mut v: Vec<P<SIZE, T>> = vec![P::<SIZE, T>::zero(); 2];
    let mut rng: ThreadRng = rand::thread_rng();

    for _ in 0..rng.gen::<u8>() {
        v.push(P::<SIZE, T>::from_slice(&rng.gen()));
    }

    for vi in v.iter() {
        println!("{:?}", vi);
    };

    for idx in 0..v.len() {
        let ptx1: P<SIZE, T> = v[idx];
        let ptx2: P<SIZE, T> = v[idx.saturating_sub(1)];
        let euclidean_distance: f64 = ptx1.euclidean_distance_to(ptx2);
        let point_distance: [T; SIZE] = ptx1.distances_to(ptx2);
        println!("distance moved from {ptx1:?} to {ptx2:?}:\n\t- {euclidean_distance}\n\t- {point_distance:?}");
    };
}
