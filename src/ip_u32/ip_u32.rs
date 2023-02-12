#![allow(non_snake_case)]

mod ip_u32_lib;

use crate::ip_u32_lib::{Octet, ToInt32, ToOctet};

use rand::{thread_rng, Rng, rngs::ThreadRng};

fn main() {
    let mut tr: ThreadRng = thread_rng();
    let num: u32 = tr.gen();
    
    let octet: Octet = num.to_octet();
    let ip:    u32   = octet.to_int32();
    
    println!("{octet:?} == {x}.{y}.{w}.{z} = {ip} === {num}", x=octet[0], y=octet[1], w=octet[2], z=octet[3]);
}

#[cfg(tests)]
mod tests {
    use std::{net::Ipv4Addr, str::FromStr};
    use super::lib::ip_u32_strucs::{int2octet, octet2int};

    #[test]
    fn t_int2octet() {
        let mut tr = thread_rng();
        let rnd: u8 = tr.gen();

        for _ in 0..rnd {
            let ip: u32 = tr.gen();
            let (x, y, w, z): (u8, u8, u8, u8) = int2octet(ip);
            
            let iip: Ipv4Addr = Ipv4Addr::from(ip);
            let oip: Ipv4Addr = Ipv4Addr::new(x, y, w, z);

            assert_eq!(iip, oip);
            println!("{} === {}", iip, oip)
        };
    }
    
    #[test]
    fn t_octet2int() {
        let mut tr = thread_rng();
        let rnd: u8 = tr.gen();

        for _ in 0..rnd {
            let octet: (u8, u8, u8, u8) = (tr.gen(), tr.gen(), tr.gen(), tr.gen());
            let ip: u32 = octet2int(octet);

            let iip: Ipv4Addr = Ipv4Addr::from(ip);
            let oip: Ipv4Addr = Ipv4Addr::from_str(octet[0], octet[1], octet[2], octet[3]);

            assert_eq!(iip, oip);
            println!("{} === {}", iip, oip)
        };
    }
}