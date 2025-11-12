use netdec::{
  matches_octet,
  matches_ip,
  matches_cidr_prefix,
  matches_range_cidr,
  matches_range_hyphen,
  matches_range_wildcard,
  matches_range_any
};

// ===== octet tests =====

#[test]
fn octet_valid() {
  for s in ["0", "5", "9", "10", "42", "99", "100", "199", "249", "255"] {
    assert!(matches_octet(s), "should accept {s}");
  }
}

#[test]
fn octet_invalid() {
  for s in ["256", "300", "-1", "01", "a", "", "9999"] {
    assert!(!matches_octet(s), "should reject {s}");
  }
}

// ===== ip tests =====

#[test]
fn ip_valid() {
  for s in ["0.0.0.0", "8.8.8.8", "192.168.1.1", "255.255.255.255"] {
    assert!(matches_ip(s), "should accept {s}");
  }
}

#[test]
fn ip_invalid() {
  for s in ["256.0.0.1", "1.2.3", "1.2.3.4.5", "1.2.3.-1", "01.2.3.4", "a.b.c.d"] {
    assert!(!matches_ip(s), "should reject {s}");
  }
}

// ===== cidr notation tests =====

#[test]
fn cidr_prefix_valid() {
  for s in ["0", "1", "8", "16", "24", "30", "31", "32"] {
    assert!(matches_cidr_prefix(s), "prefix {s} should be valid");
  }
}

#[test]
fn cidr_valid() {
  for s in ["0.0.0.0/0", "10.0.0.0/8", "192.168.0.0/16", "203.0.113.7/32"] {
    assert!(matches_range_cidr(s), "should accept {s}");
  }
}
#[test]
fn cidr_prefix_invalid() {
  for s in ["33", "100", "-1", "09", ""] {
    assert!(!matches_cidr_prefix(s), "prefix {s} should be invalid");
  }
}

#[test]
fn cidr_invalid() {
  for s in ["256.0.0.1/24", "192.168.0.0/33", "192.168.0.0/", "192.168.0.0/08"] {
    assert!(!matches_range_cidr(s), "should reject {s}");
  }
}

// ===== hyphen notation tests =====

#[test]
fn hyphen_valid() {
  for s in ["192.168.0.1-192.168.0.10", "0.0.0.0-255.255.255.255", "10.0.0.1-10.0.0.1"] {
    assert!(matches_range_hyphen(s), "should accept {s}");
  }
}

#[test]
fn hyphen_invalid() {
  for s in ["192.168.0.1-", "-192.168.0.10", "256.0.0.1-1.1.1.1", "a-b"] {
    assert!(!matches_range_hyphen(s), "should reject {s}");
  }
}

// ===== wildcard notation tests =====

#[test]
fn wildcard_valid() {
  for s in ["*", "10.*", "10.0.*", "10.0.0.*", "203.0.113.*"] {
    assert!(matches_range_wildcard(s), "should accept {s}");
  }
}

#[test]
fn wildcard_invalid() {
  for s in ["10", "10.*.*.*.*", "10.*.", ".10.*", "192.168.*.*", "10.256.*", "10.*.1"] {
    assert!(!matches_range_wildcard(s), "should reject {s}");
  }
}

// ===== general rule tests =====

#[test]
fn any_valid() {
  for s in ["192.168.0.0/16", "192.168.0.1-192.168.0.10", "10.*", "10.0.*", "10.0.0.*"] {
    assert!(matches_range_any(s), "should accept {s}");
  }
}

#[test]
fn any_invalid() {
  for s in ["", "nope", "256.0.0.1", "10.0.0.0/33", "1.2.3", "10.*.1"] {
    assert!(!matches_range_any(s), "should reject {s}");
  }
}