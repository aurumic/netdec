use std::net::Ipv4Addr;
use netdec::IpRange;

// ===== cidr notation parsing tests =====

#[test]
fn cidr_basic() {
  let r: IpRange = "192.168.0.0/24".parse().unwrap();
  assert_eq!(r.first, Ipv4Addr::new(192, 168, 0, 0));
  assert_eq!(r.last, Ipv4Addr::new(192, 168, 0, 255));
  assert_eq!(r.network, Some(Ipv4Addr::new(192, 168, 0, 0)));
  assert_eq!(r.broadcast, Some(Ipv4Addr::new(192, 168, 0, 255)));
  assert_eq!(r.prefix, Some(24));
  assert_eq!(r.netmask, Some(Ipv4Addr::new(255, 255, 255, 0)));
  assert_eq!(r.hostmask, Some(Ipv4Addr::new(0, 0, 0, 255)));
  assert_eq!(r.size, 256);
  assert!(r.contains(Ipv4Addr::new(192, 168, 0, 42)));
  assert!(!r.contains(Ipv4Addr::new(192, 168, 1, 1)));
}

#[test]
fn cidr_edge_cases() {
  let world: IpRange = "0.0.0.0/0".parse().unwrap();
  assert_eq!(world.first, Ipv4Addr::new(0, 0, 0, 0));
  assert_eq!(world.last, Ipv4Addr::new(255, 255, 255, 255));
  assert_eq!(world.prefix, Some(0));
  assert_eq!(world.size, 1u128 << 32);
  assert!(world.contains(Ipv4Addr::new(1, 2, 3, 4)));

  let single: IpRange = "8.8.8.8/32".parse().unwrap();
  assert_eq!(single.first, Ipv4Addr::new(8, 8, 8, 8));
  assert_eq!(single.last, Ipv4Addr::new(8, 8, 8, 8));
  assert_eq!(single.prefix, Some(32));
  assert_eq!(single.size, 1);
  assert!(single.contains(Ipv4Addr::new(8, 8, 8, 8)));
  assert!(!single.contains(Ipv4Addr::new(8, 8, 8, 7)));
}
