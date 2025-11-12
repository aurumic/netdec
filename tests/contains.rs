use std::net::Ipv4Addr;
use netdec::IpRange;

// ===== ip membership within given range tests =====

#[test]
fn membership_basic() {
  let r: IpRange = "192.168.1.250-192.168.2.5".parse().unwrap();
  for ip in [
    Ipv4Addr::new(192, 168, 1, 250),
    Ipv4Addr::new(192, 168, 2, 5),
  ] {
    assert!(r.contains(ip), "expected {ip} to be inside the range");
  }
  for ip in [
    Ipv4Addr::new(192, 168, 1, 249),
    Ipv4Addr::new(192, 168, 2, 6),
  ] {
    assert!(!r.contains(ip), "expected {ip} to be outside the range");
  }
}

#[test]
fn membership_edge_cases() {
  let single: IpRange = "203.0.113.9/32".parse().unwrap();
  assert!(single.contains("203.0.113.9".parse().unwrap()));
  assert!(!single.contains("203.0.113.8".parse().unwrap()));

  let world: IpRange = "0.0.0.0/0".parse().unwrap();
  for ip in ["0.0.0.0", "255.255.255.255", "8.8.8.8"] {
    assert!(world.contains(ip.parse().unwrap()), "should contain {ip}");
  }
}
