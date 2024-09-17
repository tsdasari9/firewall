use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

// Represents the NAT Table mapping internal (private) IP addresses to external (public) IP addresses and ports.
pub struct NatTable {
    pub_to_private: HashMap<SocketAddr, SocketAddr>, // Mapping of public to private addresses
    private_to_pub: HashMap<SocketAddr, SocketAddr>, // Mapping of private to public addresses
    next_port: u16,                                  // Port increment for new public ports
    public_ip: Ipv4Addr,                             // The public IP address used for NAT
}

impl NatTable {
    pub fn new(public_ip: Ipv4Addr) -> NatTable {
        NatTable {
            pub_to_private: HashMap::new(),
            private_to_pub: HashMap::new(),
            next_port: 10000, // Start assigning ports from 10000 upwards
            public_ip,
        }
    }

    // Function to translate a private address to a public one
    pub fn translate(&mut self, private_addr: SocketAddr) -> SocketAddr {
        // If the private address is already mapped, return the public address
        if let Some(pub_addr) = self.private_to_pub.get(&private_addr) {
            *pub_addr
        } else {
            // If not, create a new mapping
            let public_addr = SocketAddr::V4(SocketAddrV4::new(self.public_ip, self.next_port));
            self.next_port += 1;

            self.private_to_pub.insert(private_addr, public_addr);
            self.pub_to_private.insert(public_addr, private_addr);

            public_addr
        }
    }

    // Reverse translation: from public to private address
    pub fn reverse_translate(&self, public_addr: SocketAddr) -> Option<SocketAddr> {
        self.pub_to_private.get(&public_addr).copied()
    }
}
