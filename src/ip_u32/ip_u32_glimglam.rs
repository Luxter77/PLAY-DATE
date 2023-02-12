#![allow(non_snake_case)]

mod ip_u32_lib;

use crate::ip_u32_lib::{Randomizable, Octet};

#[inline(always)]
fn s() {
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    std::thread::sleep(std::time::Duration::from_secs_f32(1.2));
}

fn main() {
    let octet:   Octet = Octet::new_rand();

    let octet_0: u8    = octet[0];
    let octet_1: u8    = octet[1];
    let octet_2: u8    = octet[2];
    let octet_3: u8    = octet[3];

    let x:       u32   = (octet_0 as u32) << 00;
    let y:       u32   = (octet_1 as u32) << 08;
    let z:       u32   = (octet_2 as u32) << 16;
    let w:       u32   = (octet_3 as u32) << 24;

    let xy:      u32   = x   + y;
    let xyz:     u32   = xy  + z;
    let xyzw:    u32   = xyz + w;

    print!("\rpub fn octet2int(octet: Octet) -> u32 {{                                      "); s();
    print!("\rpub fn octet2int(octet = {octet:?}) -> u32 {{                                 "); s();
    print!("\n   x = (octet[0] as u32) << 00;                                               "); s();
    print!("\r   x = ({octet:?}[0] as u32) << 00;                                           "); s();
    print!("\r   x = ([{octet_0}, _, _, _][0] as u32) << 00;                                "); s();
    print!("\r   x = ({octet_0} as u32) << 00;                                              "); s();
    print!("\r   x = ({octet_0}) << 00;                                                     "); s();
    print!("\r   x = {octet_0} << 00;                                                       "); s();
    print!("\r   x = {octet_0};                                                             "); s();
    print!("\n   y = (octet[1] as u32) << 08;                                               "); s();
    print!("\r   y = ({octet:?}[1] as u32) << 08;                                           "); s();
    print!("\r   y = ([_, {octet_1}, _, _][1] as u32) << 08;                                "); s();
    print!("\r   y = ({octet_1}) << 08;                                                     "); s();
    print!("\r   y = {octet_1} << 08;                                                       "); s();
    print!("\r   y = {y};                                                                   "); s();
    print!("\n   z = (octet[2] as u32) << 16;                                               "); s();
    print!("\r   z = ({octet:?}[2] as u32) << 16;                                           "); s();
    print!("\r   z = ([_, _, {octet_2}, _][2] as u32) << 16;                                "); s();
    print!("\r   z = ({octet_2}) << 16;                                                     "); s();
    print!("\r   z = {octet_2} << 16;                                                       "); s();
    print!("\r   z = {z};                                                                   "); s();
    print!("\n   w = (octet[3] as u32) << 24;                                               "); s();
    print!("\r   w = ({octet:?}[3] as u32) << 24;                                           "); s();
    print!("\r   w = ([_, _, _, {octet_3}][3] as u32) << 24;                                "); s();
    print!("\r   w = ({octet_3}) << 24;                                                     "); s();
    print!("\r   w = {octet_3} << 24;                                                       "); s();
    print!("\r   w = {w};                                                                   "); s();
    print!("\n   return (x + y + z + w);                                                    "); s();
    print!("\r   return ({x} + y + z + w);                                                  "); s();
    print!("\r   return ({x} + {y} + z + w);                                                "); s();
    print!("\r   return ({x} + {y} + {z} + w);                                              "); s();
    print!("\r   return ({x} + {y} + {z} + {w});                                            "); s();
    print!("\r   return ({xy} + {z} + {w});                                                 "); s();
    print!("\r   return ({xyz} + {w});                                                      "); s();
    print!("\r   return ({xyzw});                                                           "); s();
    print!("\r   return {xyzw};                                                             \n"); s();
    print!("}}\n"); s();
}
