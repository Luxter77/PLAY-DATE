#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_import_braces)]
use std::io::Write;

type GenSize          = u32;
type ListPart         = [bool; GenSize::MAX as usize];
type ExtendedList<'a> = [&'a mut ListPart; 8];

const A_PRIMA: GenSize = 1103515245 as GenSize;
const C_PRIMA: GenSize = 12345;
const M_PRIMA: GenSize = GenSize::MAX;

#[cfg(feature = "HUGE-COMPUTER")] static mut PART1:  ListPart = [false; GenSize::MAX as usize];
#[cfg(feature = "HUGE-COMPUTER")] static mut PART2:  ListPart = [false; GenSize::MAX as usize];
#[cfg(feature = "HUGE-COMPUTER")] static mut PART3:  ListPart = [false; GenSize::MAX as usize];
#[cfg(feature = "HUGE-COMPUTER")] static mut PART4:  ListPart = [false; GenSize::MAX as usize];
#[cfg(feature = "HUGE-COMPUTER")] static mut PART5:  ListPart = [false; GenSize::MAX as usize];
#[cfg(feature = "HUGE-COMPUTER")] static mut PART6:  ListPart = [false; GenSize::MAX as usize];
#[cfg(feature = "HUGE-COMPUTER")] static mut PART7:  ListPart = [false; GenSize::MAX as usize];
#[cfg(feature = "HUGE-COMPUTER")] static mut PART8:  ListPart = [false; GenSize::MAX as usize];

#[cfg(feature = "HUGE-COMPUTER")] 
static mut BIG: ExtendedList = unsafe {[
        &mut PART1, &mut PART2,  &mut PART3,  &mut PART4,
        &mut PART5, &mut PART6,  &mut PART7,  &mut PART8,
]};

fn main() {
    let mut num:   GenSize = 12345;
    let mut count: GenSize = 0;
    
    #[cfg(    feature = "HUGE-COMPUTER") ] let mut round: usize = 0;
    #[cfg(not(feature = "HUGE-COMPUTER"))] let round:     usize = 0; 
    
    let mut max: GenSize;
    let mut min: GenSize;
    
    let a = A_PRIMA % M_PRIMA;
    let b = C_PRIMA % M_PRIMA;

    num = (a.overflowing_mul(num % M_PRIMA).0 % M_PRIMA).overflowing_add(b).0;
    
    let first: GenSize = num;

    max = num;
    min = num;

    loop {
        num = (a.overflowing_mul(num % M_PRIMA).0 % M_PRIMA).overflowing_add(b).0;
        
        println!("{}", format!("[ {:>3.2}% ][ num: {num}][ round: {} ]", count as f32 * 100.0 / 2147483647.0, round+1));
        std::io::stdout().flush().unwrap();

        count += 1;

        #[cfg(feature = "HUGE-COMPUTER")]
        if unsafe { BIG[round][num as usize] } {
            match round { 7 => { break; }, _ => { round += 1; } };
        } else {
            unsafe { BIG[round][num as usize] = true };
        };

        
        if ( num * (round as GenSize) ) > max { max = num * (round as GenSize) };
        if ( num * (round as GenSize) ) < min { min = num * (round as GenSize) };
        
        #[cfg(not(feature = "HUGE-COMPUTER"))] break;
    };

    let distance = max - min;

    println!("First: {first}\nCount: {count}\nMax: {max}\nMin: {min}\nDistance: {distance}");
}
