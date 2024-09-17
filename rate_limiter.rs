use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    allowed_requests: u64,
    time_window: Duration,
    clients: HashMap<IpAddr, (u64, Instant)>,
}

impl RateLimiter {
    // Constructor for RateLimiter
    pub fn new(allowed_requests: u64, time_window_seconds: u64) -> Self {
        RateLimiter {
            allowed_requests,
            time_window: Duration::new(time_window_seconds, 0),
            clients: HashMap::new(),
        }
    }

    // Method to check if a request is allowed for a given IP address
    pub fn allow_request(&mut self, ip: IpAddr) -> bool {
        let now = Instant::now();
        let entry = self.clients.entry(ip).or_insert((0, now));

        // Reset the count if the time window has expired
        if now.duration_since(entry.1) > self.time_window {
            entry.0 = 0;
            entry.1 = now;
        }

        // Check if the request is within the allowed limit
        if entry.0 < self.allowed_requests {
            entry.0 += 1;
            true
        } else {
            false
        }
    }
}
