use std::net::Ipv4Addr;
use netdec::IpRange;

// ===== wildcard notation parsing tests =====

#[test]
fn wildcard_prefixes() {
  let star: IpRange = "*".parse().unwrap();
  assert_eq!(star.first, Ipv4Addr::new(0, 0, 0, 0));
  assert_eq!(star.last, Ipv4Addr::new(255, 255, 255, 255));
  assert_eq!(star.prefix, Some(0));
  assert_eq!(star.size, 1u128 << 32);

  let a: IpRange = "10.*".parse().unwrap();
  assert_eq!(a.prefix, Some(8));
  assert_eq!(a.network, Some(Ipv4Addr::new(10, 0, 0, 0)));
  assert_eq!(a.broadcast, Some(Ipv4Addr::new(10, 255, 255, 255)));
  assert!(a.contains(Ipv4Addr::new(10, 1, 2, 3)));
  assert!(!a.contains(Ipv4Addr::new(11, 0, 0, 0)));

  let ab: IpRange = "192.168.*".parse().unwrap();
  assert_eq!(ab.prefix, Some(16));
  assert_eq!(ab.netmask, Some(Ipv4Addr::new(255, 255, 0, 0)));

  let abc: IpRange = "203.0.113.*".parse().unwrap();
  assert_eq!(abc.prefix, Some(24));
  assert_eq!(abc.size, 256);
  assert!(abc.contains(Ipv4Addr::new(203, 0, 113, 1)));
  assert!(!abc.contains(Ipv4Addr::new(203, 0, 114, 1)));
}
