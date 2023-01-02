fn main() {
    println!("|{}|", "¯".repeat(102));
    println!("| i8:    min:{:>41} | max:{:>41} |", i8::MIN,      i8::MAX   );
    println!("| u8:    min:{:>41} | max:{:>41} |", u8::MIN,      u8::MAX   );
    println!("| i16:   min:{:>41} | max:{:>41} |", i16::MIN,     i16::MAX  );
    println!("| u16:   min:{:>41} | max:{:>41} |", u16::MIN,     u16::MAX  );
    println!("| i32:   min:{:>41} | max:{:>41} |", i32::MIN,     i32::MAX  );
    println!("| u32:   min:{:>41} | max:{:>41} |", u32::MIN,     u32::MAX  );
    println!("| i64:   min:{:>41} | max:{:>41} |", i64::MIN,     i64::MAX  );
    println!("| isize: min:{:>41} | max:{:>41} |", isize::MIN,   isize::MAX);
    println!("| u64:   min:{:>41} | max:{:>41} |", u64::MIN,     u64::MAX  );
    println!("| usize: min:{:>41} | max:{:>41} |", usize::MIN,   usize::MAX);
    println!("| i128:  min:{:>41} | max:{:>41} |", i128::MIN,    i128::MAX );
    println!("| u128:  min:{:>41} | max:{:>41} |", u128::MIN,    u128::MAX );
    println!("|{}|", "_".repeat(102));


    println!("| i8:    mid:{:>41} |", i8::MAX    / 2);
    println!("| u8:    mid:{:>41} |", u8::MAX    / 2);
    println!("| i16:   mid:{:>41} |", i16::MAX   / 2);
    println!("| u16:   mid:{:>41} |", u16::MAX   / 2);
    println!("| i32:   mid:{:>41} |", i32::MAX   / 2);
    println!("| u32:   mid:{:>41} |", u32::MAX   / 2);
    println!("| i64:   mid:{:>41} |", i64::MAX   / 2);
    println!("| isize: mid:{:>41} |", isize::MAX / 2);
    println!("| u64:   mid:{:>41} |", u64::MAX   / 2);
    println!("| usize: mid:{:>41} |", usize::MAX / 2);
    println!("| i128:  mid:{:>41} |", i128::MAX  / 2);
    println!("| u128:  mid:{:>41} |", u128::MAX  / 2);
    println!("|{}|", "_".repeat(54));

}