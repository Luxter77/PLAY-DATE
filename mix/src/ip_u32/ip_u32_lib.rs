
use rand::{thread_rng, Rng, rngs::ThreadRng};

pub type Octet = [u8; 4];

// pub trait DisplayableOctet { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), &str>; }
// impl DisplayableOctet for Octet {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), &str> { Ok(()) }
// }

pub trait ToOctet { fn to_octet(self) -> Octet; }
impl ToOctet for u32 {
    fn to_octet(self) -> Octet {
        return [((self >> 00) & 0xFF) as u8, ((self >> 08) & 0xFF) as u8, ((self >> 16) & 0xFF) as u8, ((self >> 24) & 0xFF) as u8];
    }
}

pub trait ToInt32 { fn to_int32(self) -> u32; }
impl ToInt32 for Octet {
    fn to_int32(self) -> u32 {
        return ((self[0] as u32) << 00) + ((self[1] as u32) << 08) + ((self[2] as u32) << 16) + ((self[3] as u32) << 24);
    }
}

pub trait Randomizable { fn new_rand() -> Self; }
impl Randomizable for Octet {
    fn new_rand() -> Self {
        let mut tr: ThreadRng = thread_rng();
        return [tr.gen(), tr.gen(), tr.gen(), tr.gen()];
    }
}