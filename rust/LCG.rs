#![allow(dead_code)]

const A_PRIMA: u128 = 1103515245;
const C_PRIMA: u128 = 1;
const M_PRIMA: u128 = i32::MAX as u128;

fn main() {
    let mut num: u128 = 2;
    let mut max: u128;
    let mut min: u128;
    let mut count: u128 = 0;
    let first: u128;

    let a = A_PRIMA % M_PRIMA;
    let b = C_PRIMA % M_PRIMA;

    let mut stupid_big_one: [bool; u64::MAX as usize] = [false; u64::MAX as usize];
    let mut stupid_big_two: [bool; u64::MAX as usize] = [false; u64::MAX as usize];

    num = ((a * (num % M_PRIMA)) % M_PRIMA) + b;

    first = num;
    max = num;
    min = num;

    loop {
        num = ((a * (num % M_PRIMA)) % M_PRIMA) + b;
        count += 1;

        if num < u64::MAX.into() {
            if stupid_big_one[num as usize] {
                println!("{stupid_big_one:?}");
                break;
            } else {
                stupid_big_one[num as usize] = true;
            };
        } else {
            if stupid_big_two[(num - (u64::MAX as u128)) as usize] {
                println!("{stupid_big_two:?}");
                break;
            } else {
                stupid_big_two[num as usize] = true;
            };
        };

        if num > max { max = num };
        if num < min { min = num };
        // print!("{:>3.2}%\r", count as f32 * 100.0 / 2147483647.0);
    };
    let distance = max - min;

    println!("\nCount: {count}\nMax: {max}\nMin: {min}\nDistance: {distance}");
}