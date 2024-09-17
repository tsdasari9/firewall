mod firewall;
mod nat;
mod acl;
mod rate_limiter;
mod traffic_shaper;
mod idps;

use log::{info, error};
use pnet::datalink::{self, Channel};
use std::net::{IpAddr, Ipv4Addr};

fn main() {
    // Initialize the logger
    env_logger::init();

    // Select the network interface (usually 'eth0' or similar for Linux)
    let interface_name = "eth0"; // Change this to your interface name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("Could not find interface");

    // Create a new channel for packet capture
    let channel = datalink::channel(&interface, Default::default())
        .expect("Failed to create datalink channel");

    // Initialize firewall components
    let public_ip = Ipv4Addr::new(203, 0, 113, 1); // Example public IP address
    let mut nat_table = nat::NatTable::new(public_ip);

    let mut acl_rule = acl::AclRule::new();
    let mut idps = idps::Idps::new(100, 100, 60); // Adjust thresholds as needed
    let mut rate_limiter = rate_limiter::RateLimiter::new(100, 60); // 100 requests per minute
    let traffic_shaper = traffic_shaper::TrafficShaper::new();

    // Add a sample ACL rule
    let real_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
    acl_rule.add_rule(real_ip, vec![22, 443]);

    info!("Firewall started. Listening on interface: {}", interface_name);

    // Main packet processing loop
    if let Channel::Ethernet(_, mut rx) = channel {
        loop {
            match rx.next() {
                Ok(packet) => {
                    firewall::handle_packet(packet, &mut nat_table, &acl_rule, &mut idps, &mut rate_limiter, &traffic_shaper);
                },
                Err(e) => {
                    error!("Failed to read packet: {}", e);
                }
            }
        }
    } else {
        error!("Unhandled channel type");
    }
}