#![allow(non_snake_case)]

use rand::{thread_rng, Rng, rngs::ThreadRng};

fn main() {
    let mut tr: ThreadRng = thread_rng();

    let (x, y, w, z): (u8, u8, u8, u8) = (tr.gen(), tr.gen(), tr.gen(), tr.gen());
    
    let ip: u32 = octet2int((x, y, w, z));
    
    let octet: (u8, u8, u8, u8) = int2octet(ip);
    
    println!("{x}.{y}.{w}.{z} = {ip} === {octet:?}");
}

fn int2octet(int: u32) -> (u8, u8, u8, u8) {
    return ((((int as u32) >> 00) & 0xFF) as u8, (((int as u32) >> 08) & 0xFF) as u8, (((int as u32) >> 16) & 0xFF) as u8, (((int as u32) >> 24) & 0xFF) as u8);
}

fn octet2int(octet: (u8, u8, u8, u8)) -> u32 {
    return ((octet.0 as u32) << 00) + ((octet.1 as u32) << 08) + ((octet.2 as u32) << 16) + ((octet.3 as u32) << 24);
}

#[cfg(tests)]
mod tests {
    use std::{net::Ipv4Addr, str::FromStr};
    use super::{int2octet, octet2int};

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