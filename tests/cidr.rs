use netdec::{matches_cidr_prefix, matches_range_cidr};

#[test]
fn cidr_prefix_accepts_0_to_32() {
  for s in ["0", "1", "8", "16", "24", "30", "31", "32"] {
    assert!(matches_cidr_prefix(s), "prefix {s} should be valid");
  }
}

#[test]
fn cidr_prefix_rejects_out_of_range() {
  for s in ["33", "100", "-1", "09", ""] {
    assert!(!matches_cidr_prefix(s), "prefix {s} should be invalid");
  }
}

#[test]
fn cidr_accepts_valid() {
  for s in ["0.0.0.0/0", "10.0.0.0/8", "192.168.0.0/16", "203.0.113.7/32"] {
    assert!(matches_range_cidr(s), "should accept {s}");
  }
}

#[test]
fn cidr_rejects_invalid() {
  for s in ["256.0.0.1/24", "192.168.0.0/33", "192.168.0.0/", "192.168.0.0/08"] {
    assert!(!matches_range_cidr(s), "should reject {s}");
  }
}
