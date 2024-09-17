use std::net::{IpAddr, Ipv4Addr};
use std::collections::HashMap;

pub struct AclRule {
    allowed_ips: HashMap<IpAddr, Vec<u16>>, // IP address -> allowed ports
}

impl AclRule {
    pub fn new() -> AclRule {
        AclRule {
            allowed_ips: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, ip: IpAddr, ports: Vec<u16>) {
        self.allowed_ips.insert(ip, ports);
    }

    pub fn is_allowed(&self, src_ip: IpAddr, dst_port: u16) -> bool {
        if let Some(ports) = self.allowed_ips.get(&src_ip) {
            ports.contains(&dst_port)
        } else {
            false // Default behavior is to deny if no rule exists
        }
    }
}
