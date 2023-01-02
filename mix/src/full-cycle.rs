#![allow(non_snake_case)]

fn main() {
    let (mut forward, mut backward): (Mock, Mock) = (Mock::zero(), Mock::full());
    loop {
        forward.next(); backward.next();
        println!("| F: {} | B: {} |", forward.ip(), backward.ip());
    }
}

/// IPCOMP(addr, n) ((addr >> (24 - 8 * n)) & 0xFF)
/// 
/// pos_0: XN.__.__.__
/// pos_1: __.YN.__.__
/// pos_2: __.__.ZN.__
/// pos_3: __.__.__.WN
/// _: Err(msg)
/// 
fn ipcomp(las: u32, pos: u8) -> Result<u8, &'static str> {
    if pos > 3 {
        return Result::Err("u dum dum, ip adresses only have 4 members");
    } else {
        return Result::Ok((las >> (24 - (8 * pos)) & 0xFF) as u8);
    };
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Backward,
    Forward,
}

#[derive(Debug, PartialEq)]
pub struct IP {
    integer: u32,
    octets:  (u8, u8, u8, u8)
}


impl std::fmt::Display for IP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "IP {{ integer: {integer: >10}, octets: ({x: >3}, {y: >3}, {z: >3}, {w: >3}) }}",
            integer=self.integer,
            x=self.octets.0,
            y=self.octets.1,
            z=self.octets.2,
            w=self.octets.3
        )
    }
}

/// # M.O.C.K.
/// 
/// {xn}.yn.zn.wn 
#[derive(Debug)]
pub struct Mock {
    pub dir: Direction,
    pub las: u32,
    pub cn:  u32,
    pub xn:  u8, // XN.__.__.__ COMP: 3
    pub yn:  u8, // __.YN.__.__ COMP: 2
    pub zn:  u8, // __.__.ZN.__ COMP: 1
    pub wn:  u8, // __.__.__.WN COMP: 0
}

impl Mock {
    pub fn skip(&mut self, skip: u128) {
        let skip: u32 = skip.try_into().unwrap_or(u32::MAX);

        self.cn = self.cn.saturating_add(skip);

        self.las = match self.dir {
            Direction::Backward => self.las.saturating_sub(skip),
            Direction::Forward  => self.las.saturating_add(skip),
        };
        
        self.reg_from_n();
    }

    fn reg_from_n(&mut self) {
        self.las = ((self.xn as u32) << 00) + ((self.yn as u32) << 08) + ((self.zn as u32) << 16) + ((self.wn as u32) << 24);
    }

    fn reg_from_las(&mut self) {
        self.xn = ipcomp(self.las, 0).unwrap();
        self.yn = ipcomp(self.las, 1).unwrap();
        self.zn = ipcomp(self.las, 2).unwrap();
        self.wn = ipcomp(self.las, 3).unwrap();
    }

    pub fn next(&mut self) /* -> GeneratedNumber */ {
        let looped: bool;

        (self.cn, _) = self.cn.overflowing_add(1);
        
        (self.las, looped) = match self.dir {
            Direction::Forward  => self.las.overflowing_add(1),
            Direction::Backward => self.las.overflowing_sub(1),
        };
        
        self.reg_from_las();

        if looped {
            /* return GeneratedNumber::Looped(self.las) */
        } else {
            /* return GeneratedNumber::Normal(self.las) */
        };
    }
    
    pub fn previous(&mut self) {
        let looped: bool;

        self.cn = self.cn.saturating_sub(1);
        
        (self.las, looped) = match self.dir {
            Direction::Forward  => self.las.overflowing_sub(1),
            Direction::Backward => self.las.overflowing_add(1),
        };
        
        if looped {
            /* return GeneratedNumber::Looped(self.las) */
        } else {
            /* return GeneratedNumber::Normal(self.las) */
        };

        self.reg_from_las();
    }
    
    pub fn zero() -> Self {
        Self {
            las: 0,
            cn:  0,
            xn:  0,
            yn:  0,
            zn:  0,
            wn:  0,
            dir: Direction::Forward,
        }
    }

    pub fn full() -> Self {
        Self {
            las: u32::MAX,
            cn:  0,
            xn:  255,
            yn:  255,
            zn:  255,
            wn:  255,
            dir: Direction::Backward,
        }
    }

    pub fn ip(&self) -> IP {
        return IP {
            integer: self.las,
            octets: (
                self.xn, // XN.__.__.__
                self.yn, // __.YN.__.__
                self.zn, // __.__.ZN.__
                self.wn, // __.__.__.WN
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use crate::*;

    /// BOILERPLATE
    fn int2octet(int: u32) -> [u8; 4] {
        return [(((int as u32) >> 24) & 0xFF) as u8, (((int as u32) >> 16) & 0xFF) as u8, (((int as u32) >> 08) & 0xFF) as u8, (((int as u32) >> 00) & 0xFF) as u8];
    }

    trait ToTouple4 { fn touple4u8(&self) -> (u8, u8, u8, u8); }
    impl ToTouple4 for [u8; 4] { fn touple4u8(&self) -> (u8, u8, u8, u8) { return (self[0], self[1], self[2],self[3]) } }

    #[test]
    fn t_int2octet() {
        let mut rng = rand::thread_rng();
        
        let iip: u32 = rand::Rng::gen(&mut rng);
        let oip: [u8; 4] = int2octet(iip);

        assert_eq!(Ipv4Addr::from(oip), Ipv4Addr::from(iip));
    }

    #[test]
    fn mock2ip_forward() {
        let mut rng = rand::thread_rng();
        let rn: u8 = rand::Rng::gen(&mut rng);
        
        let mut m: Mock = Mock::zero();

        for _ in 0..rn { m.next(); };

        let ip: IP = m.ip();

        assert_eq!(Ipv4Addr::from(ip.integer), Ipv4Addr::new(ip.octets.0, ip.octets.1, ip.octets.2, ip.octets.3));
    }

    #[test]
    fn mock2ip_backward() {
        let mut rng = rand::thread_rng();
        let rn: u8 = rand::Rng::gen(&mut rng);
        
        let mut m: Mock = Mock::full();

        for _ in 0..rn {
            m.next();
        }

        let ip: IP = m.ip();

        assert_eq!(Ipv4Addr::from(ip.integer), Ipv4Addr::new(ip.octets.0, ip.octets.1, ip.octets.2, ip.octets.3));
    }

    #[test]
    fn mock_next() {
        let mut rng = rand::thread_rng();
        let rn: u8 = rand::Rng::gen(&mut rng);
        
        let mut m: Mock = Mock::zero();
        
        for _ in 0..rn {
            m.next();
        };
        
        assert_eq!((IP { octets: int2octet(rn as u32).touple4u8(), integer: rn as u32 }, rn as u32), (m.ip(), m.cn));
        
        for _ in 0..rn {
            m.previous();
        };

        assert_eq!(
            (m.ip(), m.cn),
            (IP { octets:(0, 0, 0, 0), integer: 0 }, 0)
        );
    }

    #[test]
    fn mock_prev() {
        let mut rng = rand::thread_rng();
        let rn: u8 = rand::Rng::gen(&mut rng);

        let mut m: Mock = Mock::full();

        for _ in 0..rn {
            m.next();
        };
        
        assert_eq!((IP { octets: int2octet(u32::MAX - rn as u32).touple4u8(), integer: u32::MAX - rn as u32}, rn as u32), (m.ip(), m.cn));
        
        for _ in 0..rn {
            m.previous();
        };
        
        assert_eq!(
            (m.ip(), m.cn),
            (IP { octets: (255, 255, 255, 255), integer: u32::MAX }, 0)
        );
    }
}
