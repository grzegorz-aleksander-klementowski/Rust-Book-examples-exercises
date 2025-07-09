#[derive(Debug)]
struct Ipv4Addr {
    octet1: u8,
    octet2: u8,
    octet3: u8,
    octet4: u8,
}

#[derive(Debug)]
struct Ipv6Addr {
    full_address: String,
}

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl Ipv4Addr {
    fn new(addr: &str) -> Option<Ipv4Addr> {
        let parts: Vec<&str> = addr.split('.').collect();
        if parts.len() != 4 {
            return None;
        }

        let octets: Option<Vec<u8>> = parts.into_iter().map(|part| part.parse().ok()).collect();

        match octets {
            Some(octets) if octets.len() == 4 => Some(Ipv4Addr {
                octet1: octets[0],
                octet2: octets[1],
                octet3: octets[2],
                octet4: octets[3],
            }),
            _ => None,
        }
    }
}

impl Ipv6Addr {
    fn new(addr: &str) -> Ipv6Addr {
        Ipv6Addr {
            full_address: addr.to_string(),
        }
    }
}

use std::fmt;

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.octet1, self.octet2, self.octet3, self.octet4
        )
    }
}

impl fmt::Display for Ipv6Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_address)
    }
}

impl Ipv4Addr {
    fn is_loopback(&self) -> bool {
        self.octet1 == 127 && self.octet2 == 0 && self.octet3 == 0 && self.octet4 == 1
    }
}

impl Ipv6Addr {
    fn is_loopback(&self) -> bool {
        self.full_address == "::1"
    }
}

fn main() {
    let home_v4 = Ipv4Addr::new("127.0.0.1").expect("Invalid IPv4 address");
    let loopback_v6 = Ipv6Addr::new("::1");

    let home = IpAddr::V4(home_v4);
    let loopback = IpAddr::V6(loopback_v6);

    match &home {
        IpAddr::V4(ip) => println!("IPv4 Address: {}, is loopback: {}", ip, ip.is_loopback()),
        _ => (),
    }

    match &loopback {
        IpAddr::V6(ip) => println!("IPv6 Address: {}, is loopback: {}", ip, ip.is_loopback()),
        _ => (),
    }
}
