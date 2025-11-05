use netdec::matches_octet;

#[test]
fn octet_accepts_valid() {
  for s in ["0", "5", "9", "10", "42", "99", "100", "199", "249", "255"] {
    assert!(matches_octet(s), "should accept {s}");
  }
}

#[test]
fn octet_rejects_invalid() {
  for s in ["256", "300", "-1", "01", "a", "", "9999"] {
    assert!(!matches_octet(s), "should reject {s}");
  }
}
