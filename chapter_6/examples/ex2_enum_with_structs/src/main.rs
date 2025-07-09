fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct Ipv4Addr {
        kind: IpAddrKind,
        address: String,
    }

    struct Ipv6Addr {
        kind: IpAddrKind,
        address: String,
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
    let home_v4 = Ipv4Addr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home_ip = IpAddr::V4(home_v4);
}
