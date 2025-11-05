use netdec::matches_ip;

#[test]
fn ip_accepts_valid() {
  for s in ["0.0.0.0", "8.8.8.8", "192.168.1.1", "255.255.255.255"] {
    assert!(matches_ip(s), "should accept {s}");
  }
}

#[test]
fn ip_rejects_invalid() {
  for s in ["256.0.0.1", "1.2.3", "1.2.3.4.5", "1.2.3.-1", "01.2.3.4", "a.b.c.d"] {
    assert!(!matches_ip(s), "should reject {s}");
  }
}
