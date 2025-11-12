use netdec::matches_range_wildcard;

#[test]
fn wildcard_accepts_valid() {
  for s in ["*", "10.*", "10.0.*", "10.0.0.*", "203.0.113.*"] {
    assert!(matches_range_wildcard(s), "should accept {s}");
  }
}

#[test]
fn wildcard_rejects_invalid() {
  for s in ["10", "10.*.*.*.*", "10.*.", ".10.*", "192.168.*.*", "10.256.*", "10.*.1"] {
    assert!(!matches_range_wildcard(s), "should reject {s}");
  }
}
