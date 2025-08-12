# Rust Firewall

This project is a medium-robust firewall written in Rust. It includes Access Control Lists (ACL), Network Address Translation (NAT), Rate Limiting, Traffic Shaping, and a placeholder Intrusion Detection and Prevention System (IDPS).

## Features
- Access Control Lists (ACL) to filter packets by IP address and port.
- Network Address Translation (NAT) to map private IP addresses to a public IP address and port.
- Rate Limiting to restrict requests per IP address within a defined time window.
- Traffic Shaping to control packet flow and bandwidth usage.
- Placeholder Intrusion Detection and Prevention System (IDPS) for future threat detection logic.
- Packet inspection and processing via the `pnet` crate.

## Project Structure

- **main.rs**: Entry point, initializes all modules and starts packet capture.
- **firewall.rs**: Core packet handling logic. Applies NAT, ACL, IDPS, Rate Limiting, and Traffic Shaping checks.
- **acl.rs**: Implements Access Control Lists (ACL).
- **nat.rs**: Implements Network Address Translation (NAT).
- **rate_limiter.rs**: Implements per-IP rate limiting.
- **traffic_shaper.rs**: Stub for traffic shaping logic.
- **idps.rs**: Stub for intrusion detection logic.

## Usage Example

```bash
cargo run --release
```

The firewall will listen on the configured network interface (`eth0` by default) and log allowed and blocked packets.

Example ACL setup in `main.rs`:

```rust
let real_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
acl_rule.add_rule(real_ip, vec![22, 443]);
```

## Dependencies
From `Cargo.toml`:
- `pnet`
- `pnet_packet`
- `log`
- `env_logger`
- `chrono`
- `tokio`

## Security Notes
- The current IDPS and Traffic Shaper modules are placeholders without real detection or shaping logic.
- Running this firewall requires elevated privileges to capture packets.
- Ensure you configure ACLs, NAT, and limits according to your environment before deploying.

## License
MIT — free to use and modify.
