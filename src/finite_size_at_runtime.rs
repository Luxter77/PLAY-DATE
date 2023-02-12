#![allow(non_snake_case)]

use std::sync::Arc;

fn main() {
    let mut v: Vec<*mut u8> = Vec::new();
    // let mut v: Vec<&Vec<&Vec<&Vec<&Vec<&Vec<&Vec<&Vec<...>>>>>>>> = Vec::new();

    let a: Arc<u32> = Arc::new(42);
    
    v.push(Arc::<u32>::as_ptr(&a) as *mut u8);
    v.push(v.as_ptr() as *mut u8);
    // v.push(&v); // Compyler would blow up here!
    
    println!("{v:?}");
}
