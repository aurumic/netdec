use std::net::Ipv4Addr;
use netdec::IpRange;

// ===== iteration tests =====

#[test]
fn iter_cidr_short() {
  let r: IpRange = "192.0.2.0/30".parse().unwrap();
  let got: Vec<Ipv4Addr> = r.iter().collect();
  assert_eq!(got.len(), 4);
  assert_eq!(got.first().unwrap(), &Ipv4Addr::new(192, 0, 2, 0));
  assert_eq!(got.last().unwrap(),  &Ipv4Addr::new(192, 0, 2, 3));
}

#[test]
fn iter_hyphen_forward_and_rev() {
  let r: IpRange = "10.0.0.9-10.0.0.12".parse().unwrap();
  let fwd: Vec<_> = r.iter().collect();
  assert_eq!(fwd, vec![
    Ipv4Addr::new(10,0,0,9),
    Ipv4Addr::new(10,0,0,10),
    Ipv4Addr::new(10,0,0,11),
    Ipv4Addr::new(10,0,0,12),
  ]);
  let rev: Vec<_> = r.iter().rev().collect();
  assert_eq!(rev, vec![
    Ipv4Addr::new(10,0,0,12),
    Ipv4Addr::new(10,0,0,11),
    Ipv4Addr::new(10,0,0,10),
    Ipv4Addr::new(10,0,0,9),
  ]);
}

#[test]
fn iter_wildcard_long() {
  // big range, only check the ends
  let r: IpRange = "203.0.113.*".parse().unwrap();
  let first3: Vec<_> = r.iter().take(3).collect();
  assert_eq!(first3, vec![
    Ipv4Addr::new(203,0,113,0),
    Ipv4Addr::new(203,0,113,1),
    Ipv4Addr::new(203,0,113,2),
  ]);
  let last2: Vec<_> = r.iter().rev().take(2).collect();
  assert_eq!(last2, vec![
    Ipv4Addr::new(203,0,113,255),
    Ipv4Addr::new(203,0,113,254),
  ]);
}

#[test]
fn into_iter_conversion() {
  let r: IpRange = "192.168.1.250-192.168.2.1".parse().unwrap();
  // by ref
  let first_ref = (&r).into_iter().next().unwrap();
  assert_eq!(first_ref, Ipv4Addr::new(192,168,1,250));
  // by value (use a clone to keep r around)
  let first_val = r.clone().into_iter().next().unwrap();
  assert_eq!(first_val, Ipv4Addr::new(192,168,1,250));
}
