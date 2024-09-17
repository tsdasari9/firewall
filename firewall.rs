use log::{info, warn, error, debug};
use pnet::packet::{ipv4::Ipv4Packet, tcp::TcpPacket, udp::UdpPacket, Packet};
use std::net::{IpAddr, SocketAddr};
use crate::{nat::NatTable, acl::AclRule, idps::Idps, rate_limiter::RateLimiter, traffic_shaper::TrafficShaper};

pub fn handle_packet(packet: &[u8], nat_table: &mut NatTable, acl_rule: &AclRule, idps: &mut Idps, rate_limiter: &mut RateLimiter, traffic_shaper: &TrafficShaper) {
    if let Some(ipv4_packet) = Ipv4Packet::new(packet) {
        let src_ip = IpAddr::V4(ipv4_packet.get_source());
        let dst_ip = IpAddr::V4(ipv4_packet.get_destination());
        let packet_size = ipv4_packet.packet().len() as u32;

        // Check TCP/UDP packet details
        let (port, is_syn) = if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
            (Some(tcp_packet.get_destination()), tcp_packet.get_flags() & 0x02 != 0) // SYN flag
        } else if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
            (Some(udp_packet.get_destination()), false)
        } else {
            (None, false)
        };

        // Intrusion detection logging
        if idps.is_intrusion(src_ip, port, is_syn) {
            warn!("Intrusion detected from IP: {}", src_ip);
            return;
        }

        // Rate limiting check
        if !rate_limiter.allow_request(src_ip) {
            warn!("Rate limit exceeded for IP: {}", src_ip);
            return;
        }

        // Traffic shaping check
        if !traffic_shaper.check_traffic(dst_ip, packet_size) {
            warn!("Traffic shaping limit exceeded for destination IP: {}", dst_ip);
            return;
        }

        // NAT translation logging
        if let Some(port) = port {
            let src_socket = SocketAddr::new(src_ip, port);
            let translated_ip = nat_table.translate(src_socket);
            info!("NAT translation: {} -> {}", src_socket, translated_ip);
        }

        // ACL check logging
        if let Some(port) = port {
            if acl_rule.is_allowed(src_ip, port) {
                info!("Packet allowed: {} -> {}:{}", src_ip, dst_ip, port);
            } else {
                warn!("Packet blocked: {} -> {}:{}", src_ip, dst_ip, port);
            }
        } else {
            debug!("Non-TCP/UDP packet received from: {}", src_ip);
        }
    } else {
        error!("Received a non-IPv4 packet");
    }
}