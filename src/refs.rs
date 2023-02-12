#![allow(non_snake_case)]

static NE: Vec<i32> = Vec::new();

fn agregar_2(vector: &mut Vec<i32>) {
    vector.push(2);
}

unsafe fn very_bad_function<T>(reference: &T) -> &mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}

fn main() {
    let x: &mut Vec<u8>;

    agregar_2(unsafe { very_bad_function(&NE) });

    let w = vec![1, 2, 3];

    println!( "evil!" );
    x = unsafe { very_bad_function(&w) };

    println!( "pushing!" );
    x.push(4);
    
    println!( "referencing!" );
    let m: &mut Vec<i32> = unsafe { very_bad_function(&NE) };
    
    println!( "printing!" );
    println!("x: {x:?}");
    println!("m: {m:?}");
}
