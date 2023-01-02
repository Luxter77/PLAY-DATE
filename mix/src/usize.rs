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
    println!("| i8:    mid:{:>41} | sqrt:{:>40} |", i8::MAX    / 2, (i8::MAX    as f64).sqrt());
    println!("| u8:    mid:{:>41} | sqrt:{:>40} |", u8::MAX    / 2, (u8::MAX    as f64).sqrt());
    println!("| i16:   mid:{:>41} | sqrt:{:>40} |", i16::MAX   / 2, (i16::MAX   as f64).sqrt());
    println!("| u16:   mid:{:>41} | sqrt:{:>40} |", u16::MAX   / 2, (u16::MAX   as f64).sqrt());
    println!("| i32:   mid:{:>41} | sqrt:{:>40} |", i32::MAX   / 2, (i32::MAX   as f64).sqrt());
    println!("| u32:   mid:{:>41} | sqrt:{:>40} |", u32::MAX   / 2, (u32::MAX   as f64).sqrt());
    println!("| i64:   mid:{:>41} | sqrt:{:>40} |", i64::MAX   / 2, (i64::MAX   as f64).sqrt());
    println!("| isize: mid:{:>41} | sqrt:{:>40} |", isize::MAX / 2, (isize::MAX as f64).sqrt());
    println!("| u64:   mid:{:>41} | sqrt:{:>40} |", u64::MAX   / 2, (u64::MAX   as f64).sqrt());
    println!("| usize: mid:{:>41} | sqrt:{:>40} |", usize::MAX / 2, (usize::MAX as f64).sqrt());
    println!("| i128:  mid:{:>41} | sqrt:{:>40} |", i128::MAX  / 2, (i128::MAX  as f64).sqrt());
    println!("| u128:  mid:{:>41} | sqrt:{:>40} |", u128::MAX  / 2, (u128::MAX  as f64).sqrt());
    println!("|{}|", "_".repeat(102));

}