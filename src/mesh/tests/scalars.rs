use crate::scalars::{SScalar, UScalar, FScalar};

use num_bigint::{BigUint, BigInt};
use num_bigfloat::BigFloat;

macro_rules! test_scalar_ops_u {
    ($n:expr, $p:expr) => {
        println!("  {n} +  {p} => {r}", n = $n, p = $p, r =   $n +  $p);
        println!("  {n} *  {p} => {r}", n = $n, p = $p, r =   $n *  $p);
        println!("  {n} /  {p} => {r}", n = $n, p = $p, r =   $n /  $p);
        println!("  {n} %  {p} => {r}", n = $n, p = $p, r =   $n %  $p);
        println!("  {n} -  {p} => {r}", n = $n, p = $p, r =   $n -  $p);
        println!("  {n} |  {p} => {r}", n = $n, p = $p, r =   $n |  $p);
        println!("  {n} &  {p} => {r}", n = $n, p = $p, r =   $n &  $p);
        println!("  {n} ^  {p} => {r}", n = $n, p = $p, r =   $n ^  $p);
    };
}

macro_rules! test_scalar_binops_u {
    ($n:expr, $p:expr) => {
        println!("  {n} << {p} => {r}", n = $n, p = $p, r =   $n << $p);
        println!("  {n} >> {p} => {r}", n = $n, p = $p, r =   $n >> $p);
        println!("! {n}        => {r}", n = $n,         r = ! $n      );
    };
}


macro_rules! test_scalar_ops_s {
    ($n:expr, $p:expr) => {
        println!("  {n} +  {p} => {r}", n = $n, p = $p, r =   $n +  $p);
        println!("  {n} *  {p} => {r}", n = $n, p = $p, r =   $n *  $p);
        println!("  {n} /  {p} => {r}", n = $n, p = $p, r =   $n /  $p);
        println!("  {n} %  {p} => {r}", n = $n, p = $p, r =   $n %  $p);
        println!("  {n} -  {p} => {r}", n = $n, p = $p, r =   $n -  $p);
        println!("- {n}        => {r}", n = $n,         r = - $n     );

        println!("  {n} |  {p} => {r}", n = $n, p = $p, r =   $n |  $p);
        println!("  {n} &  {p} => {r}", n = $n, p = $p, r =   $n &  $p);
        println!("  {n} ^  {p} => {r}", n = $n, p = $p, r =   $n ^  $p);
        println!("! {n}        => {r}", n = $n,         r = ! $n     );
    };
}


macro_rules! test_scalar_binshiftops_s {
    ($n:expr, $p:expr) => {
        println!("  {n} << {p} => {r}", n = $n, p = $p, r =   $n << $p);
        println!("  {n} >> {p} => {r}", n = $n, p = $p, r =   $n >> $p);
    };
}


macro_rules! test_scalar_ops_f {
    ($n:expr, $p:expr) => {
        println!("  {n} +  {p} => {r}", n = $n, p = $p, r =   $n + $p);
        println!("  {n} *  {p} => {r}", n = $n, p = $p, r =   $n * $p);
        println!("  {n} /  {p} => {r}", n = $n, p = $p, r =   $n / $p);
        println!("  {n} %  {p} => {r}", n = $n, p = $p, r =   $n % $p);
        println!("  {n} -  {p} => {r}", n = $n, p = $p, r =   $n - $p);
        println!("- {n}        => {r}", n = $n,         r = - $n     );
    };
}

#[test] fn test_scalar_create_u8()      {
    test_scalar_ops_u!   (UScalar(4u8), UScalar(2u8));
    test_scalar_binops_u!(UScalar(4u8), UScalar(2u8));
}

#[test] fn test_scalar_create_u16()     {
    test_scalar_ops_u!   (UScalar(4u16), UScalar(2u16));
    test_scalar_binops_u!(UScalar(4u16), UScalar(2u16));
}

#[test] fn test_scalar_create_u32()     {
    test_scalar_ops_u!   (UScalar(4u32), UScalar(2u32));
    test_scalar_binops_u!(UScalar(4u32), UScalar(2u32));
}

#[test] fn test_scalar_create_u64()     {
    test_scalar_ops_u!   (UScalar(4u64), UScalar(2u64));
    test_scalar_binops_u!(UScalar(4u64), UScalar(2u64));
}

#[test] fn test_scalar_create_u128()    {
    test_scalar_ops_u!   (UScalar(4u128), UScalar(2u128));
    test_scalar_binops_u!(UScalar(4u128), UScalar(2u128));
}

#[test] fn test_scalar_create_usize()   {
    test_scalar_ops_u!   (UScalar(4usize), UScalar(2usize));
    test_scalar_binops_u!(UScalar(4usize), UScalar(2usize));
}

#[test] fn test_scalar_create_biguint() {
    test_scalar_ops_u!(UScalar(BigUint::from(400u32)), UScalar(BigUint::from(200u32)));
    // No bin ops for BigUint.
    // test_scalar_binops_u!(UScalar(BigUint::from(400u32)), UScalar(BigUint::from(200u32)));
}

#[test] fn test_scalar_create_i8()    {
    test_scalar_ops_s!        (SScalar(4i8), SScalar(2i8));
    test_scalar_binshiftops_s!(SScalar(4i8), SScalar(2i8));
}
#[test] fn test_scalar_create_i16()   {
    test_scalar_ops_s!        (SScalar(4i16), SScalar(2i16));
    test_scalar_binshiftops_s!(SScalar(4i16), SScalar(2i16));
}
#[test] fn test_scalar_create_i32()   {
    test_scalar_ops_s!        (SScalar(4i32), SScalar(2i32));
    test_scalar_binshiftops_s!(SScalar(4i32), SScalar(2i32));
}
#[test] fn test_scalar_create_i64()   {
    test_scalar_ops_s!        (SScalar(4i64), SScalar(2i64));
    test_scalar_binshiftops_s!(SScalar(4i64), SScalar(2i64));
}
#[test] fn test_scalar_create_i128()  {
    test_scalar_ops_s!        (SScalar(4i128), SScalar(2i128));
    test_scalar_binshiftops_s!(SScalar(4i128), SScalar(2i128));
}
#[test] fn test_scalar_create_isize() {
    test_scalar_ops_s!        (SScalar(4isize), SScalar(2isize));
    test_scalar_binshiftops_s!(SScalar(4isize), SScalar(2isize));
}
#[test] fn test_scalar_create_bigint() {
    test_scalar_ops_s!(SScalar(BigInt::from(400)), SScalar(BigInt::from(200)));
    // No bin ops for BigInt.
    // test_scalar_binshiftops_s!(SScalar(BigInt::from(400)), SScalar(BigInt::from(200)));
}

#[test] fn test_scalar_create_f32()   {
    test_scalar_ops_f!(FScalar(2.4f32), FScalar(1.1f32));
}

#[test] fn test_scalar_create_f64()   {
    test_scalar_ops_f!(FScalar(2.4f64), FScalar(1.1f64));
}

#[test] fn test_scalar_create_bigfloat() {
    test_scalar_ops_f!(FScalar(BigFloat::from(400.0001)), FScalar(BigFloat::from(200.0001)));
}
