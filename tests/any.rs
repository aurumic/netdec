use netdec::matches_range_any;

#[test]
fn any_accepts_every_form() {
  for s in ["192.168.0.0/16", "192.168.0.1-192.168.0.10", "10.*", "10.0.*", "10.0.0.*"] {
    assert!(matches_range_any(s), "should accept {s}");
  }
}

#[test]
fn any_rejects_garbage() {
  for s in ["", "nope", "256.0.0.1", "10.0.0.0/33", "1.2.3", "10.*.1"] {
    assert!(!matches_range_any(s), "should reject {s}");
  }
}
