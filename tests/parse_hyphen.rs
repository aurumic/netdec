use std::net::Ipv4Addr;
use netdec::{IpRange, IpRangeError};

// ===== hyphen notation parsing tests =====

#[test]
fn hyphen_iprange_object() {
  let r: IpRange = "192.168.0.1-192.168.0.10".parse().unwrap();
  assert_eq!(r.first, Ipv4Addr::new(192, 168, 0, 1));
  assert_eq!(r.last, Ipv4Addr::new(192, 168, 0, 10));
  assert_eq!(r.size, 10);
  assert_eq!(r.prefix, None);
  assert_eq!(r.network, None);
  assert_eq!(r.broadcast, None);
  assert!(r.contains(Ipv4Addr::new(192, 168, 0, 5)));
  assert!(!r.contains(Ipv4Addr::new(192, 168, 0, 11)));
}

#[test]
fn hyphen_cidr_calulation() {
  for (range, pfx, net, bcast) in [
    ("10.0.0.0-10.0.0.255", 24u8, Ipv4Addr::new(10, 0, 0, 0), Ipv4Addr::new(10, 0, 0, 255)),
    ("172.16.0.0-172.16.255.255", 16, Ipv4Addr::new(172, 16, 0, 0), Ipv4Addr::new(172, 16, 255, 255)),
    ("192.0.2.0-192.0.2.127", 25, Ipv4Addr::new(192, 0, 2, 0), Ipv4Addr::new(192, 0, 2, 127)),
  ] {
    let r: IpRange = range.parse().unwrap();
    assert_eq!(r.prefix, Some(pfx), "prefix mismatch for {range}");
    assert_eq!(r.network, Some(net), "network mismatch for {range}");
    assert_eq!(r.broadcast, Some(bcast), "broadcast mismatch for {range}");
  }
}

#[test]
fn hyphen_invalid_range() {
  let err = "1.1.1.10-1.1.1.1".parse::<IpRange>().unwrap_err();
  assert!(matches!(err, IpRangeError::InvalidRangeOrder));
}
