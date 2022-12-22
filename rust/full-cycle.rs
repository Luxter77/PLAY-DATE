//# rand = "0.8.5"

use rand::Rng;

fn main() {
    let seed: u128 = rand::thread_rng().gen::<u16>() as u128;
    let sample_size = 4294967296u128;
    let mut generated_number = seed % sample_size;
    let increment = 65280u128;
    let mut count: u128 = 0u128;
    
    for _ in 1..=sample_size {
        println!("Last number => {}", generated_number);
        generated_number = (generated_number + increment) % sample_size;
        count += 1u128;
    }
    
    println!("We generated => {} items", count);
}