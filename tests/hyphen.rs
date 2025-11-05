use netdec::matches_range_hyphen;

#[test]
fn hyphen_accepts_valid() {
  for s in ["192.168.0.1-192.168.0.10", "0.0.0.0-255.255.255.255", "10.0.0.1-10.0.0.1"] {
    assert!(matches_range_hyphen(s), "should accept {s}");
  }
}

#[test]
fn hyphen_rejects_invalid() {
  for s in ["192.168.0.1-", "-192.168.0.10", "256.0.0.1-1.1.1.1", "a-b"] {
    assert!(!matches_range_hyphen(s), "should reject {s}");
  }
}
