use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

use std::net::Ipv4Addr;
use std::str::FromStr;
use std::iter::FusedIterator;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IpParser;

#[derive(Debug, Error)]
pub enum IpRangeError {
  #[error(transparent)]
  Pest(#[from] pest::error::Error<Rule>),

  #[error("invalid IPv4 address: {0}")]
  InvalidIp(String),

  #[error("invalid range: start > end")]
  InvalidRangeOrder,

  #[error("unsupported format")]
  UnsupportedFormat
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeKind {
  Cidr { base: Ipv4Addr, prefix: u8 },
  Hyphen { start: Ipv4Addr, end: Ipv4Addr },
  Wildcard { specified_octets: u8 }
}


// ===== core logic =====


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpRange {
  pub input: String, // original input
  pub kind: RangeKind, // kind of input that was parsed

  pub first: Ipv4Addr, // first ip address in range
  pub last: Ipv4Addr, // last ip address in range

  // only applies if ip range aligns to a single cidr block
  pub network: Option<Ipv4Addr>, // network address
  pub broadcast: Option<Ipv4Addr>, // broadcast address
  pub prefix: Option<u8>, // cidr prefix length
  pub netmask: Option<Ipv4Addr>, // network mask
  pub hostmask: Option<Ipv4Addr>, // host mask

  pub size: u128, // number of IPv4 addresses covered inside the range
}

#[derive(Clone, Debug)]
pub struct IpRangeIter {
  cur: u32,
  end: u32,
  done: bool,
}

impl IpRange {

  /*
  parse an IPv4 range within one of the supported syntaxes
  
  supports:
  - cidr: "a.b.c.d/nn"
  - hyphen: "a.b.c.d-e.f.g.h"
  - wildcard: "*", "a.*", "a.b.*", "a.b.c.*"
  */

  pub fn parse(input: &str) -> Result<Self, IpRangeError> {
    // input classification
    if full_match(Rule::range_cidr, input) {
      Self::parse_cidr(input)

    } else if full_match(Rule::range_hyphen, input) {
      Self::parse_hyphen(input)

    } else if full_match(Rule::range_wildcard, input) {
      Self::parse_wildcard(input)

    } else {
      IpParser::parse(Rule::range_any, input)?;
      Err(IpRangeError::UnsupportedFormat)
    }
  }

  fn parse_cidr(input: &str) -> Result<Self, IpRangeError> {
    // parse ip address followed by a cidr prefix, ex. 192.168.0.1/24
    let (ip_s, prefix_s) = input.split_once('/').expect("validated by grammar");
    let base = parse_ipv4(ip_s)?;
    let prefix: u8 = prefix_s.parse().unwrap();

    let mask = mask_from_prefix(prefix);
    let hostmask = !mask;
    let network = base & mask;
    let broadcast = network | hostmask;

    let first = Ipv4Addr::from(network);
    let last = Ipv4Addr::from(broadcast);

    Ok(IpRange {
      input: input.to_string(),
      kind: RangeKind::Cidr { base: Ipv4Addr::from(base), prefix },
      first,
      last,
      network: Some(first),
      broadcast: Some(last),
      prefix: Some(prefix),
      netmask: Some(Ipv4Addr::from(mask)),
      hostmask: Some(Ipv4Addr::from(hostmask)),
      size: size_from_prefix(prefix),
    })
  }

  fn parse_hyphen(input: &str) -> Result<Self, IpRangeError> {
    // parse two ip addresses joined by a hyphen, ex. 192.168.0.0-192.168.0.100
    let (start_s, end_s) = input.split_once('-').expect("validated by grammar");
    
    let start_val = parse_ipv4(start_s)?;
    let end_val = parse_ipv4(end_s)?;
    if start_val > end_val {
      return Err(IpRangeError::InvalidRangeOrder);
    }

    let first = Ipv4Addr::from(start_val);
    let last = Ipv4Addr::from(end_val);
    let size = (end_val as u128).wrapping_sub(start_val as u128) + 1;

    // attempt to find a cidr block if range is aligned
    let (network, broadcast, prefix, netmask, hostmask) = match infer_cidr_from_range(start_val, end_val) {
      Some(pfx) => {
        let mask = mask_from_prefix(pfx);
        let host = !mask;
        let net = start_val & mask;
        let bcast = net | host;
        (Some(Ipv4Addr::from(net)),
         Some(Ipv4Addr::from(bcast)),
         Some(pfx),
         Some(Ipv4Addr::from(mask)),
         Some(Ipv4Addr::from(host)))
      }
      None => (None, None, None, None, None),
    };

    Ok(IpRange {
      input: input.to_string(),
      kind: RangeKind::Hyphen { start: first, end: last },
      first,
      last,
      network,
      broadcast,
      prefix,
      netmask,
      hostmask,
      size,
    })
  }

  fn parse_wildcard(input: &str) -> Result<Self, IpRangeError> {
    // parse wildcard notation, ex. 192.168.*
    let parts: Vec<&str> = input.split('.').collect();
    let specified_octets: u8 = if input == "*" {
      0
    } else {
      (parts.len() as u8) - 1
    };

    let mut octs = [0u8; 4];
    for i in 0..(specified_octets as usize) {
      octs[i] = parts[i].parse::<u8>().map_err(|_| IpRangeError::InvalidIp(parts[i].to_string()))?;
    }

    let mut first_val: u32 = 0;
    let mut last_val: u32 = 0;

    for (i, &oct) in octs.iter().enumerate() {
      let shift = 8 * (3 - i as u32);
      first_val |= (oct as u32) << shift;
      last_val |= (oct as u32) << shift;
    }

    // fill missing octets
    for i in (specified_octets as usize)..4 {
      let shift = 8 * (3 - i as u32);
      last_val |= 255u32 << shift;
    }

    let prefix = 8 * specified_octets;
    let mask = mask_from_prefix(prefix);
    let host = !mask;

    Ok(IpRange {
      input: input.to_string(),
      kind: RangeKind::Wildcard { specified_octets },
      first: Ipv4Addr::from(first_val),
      last: Ipv4Addr::from(last_val),
      network: Some(Ipv4Addr::from(first_val)),
      broadcast: Some(Ipv4Addr::from(last_val)),
      prefix: Some(prefix),
      netmask: Some(Ipv4Addr::from(mask)),
      hostmask: Some(Ipv4Addr::from(host)),
      size: size_from_prefix(prefix),
    })
  }

  pub fn is_valid(input: &str) -> bool {
    // boolean check for whether a string is a supported IPv4 range
    Self::parse(input).is_ok()
  }

  pub fn contains(&self, ip: Ipv4Addr) -> bool {
    // boolean check for whether the range contains a given IPv4 address
    let x = u32::from(ip);
    let a = u32::from(self.first);
    let b = u32::from(self.last);
    a <= x && x <= b
  }

  pub fn iter(&self) -> IpRangeIter {
    // iterator over all ips in the range
    IpRangeIter {
      cur: u32::from(self.first),
      end: u32::from(self.last),
      done: false,
    }
  }
}

pub fn is_valid_ip_range(input: &str) -> bool {
  IpRange::is_valid(input)
}


// ===== iterator logic =====


impl Iterator for IpRangeIter {
  type Item = std::net::Ipv4Addr;

  fn next(&mut self) -> Option<Self::Item> {
    if self.done {
      return None;
    }

    let out = self.cur;
    if self.cur == self.end {
      self.done = true;
    } else {
      self.cur = self.cur.wrapping_add(1);
    }

    Some(std::net::Ipv4Addr::from(out))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    if self.done {
      return (0, Some(0));
    }

    let rem = (self.end as u128)
      .wrapping_sub(self.cur as u128)
      .wrapping_add(1);
    let lb = rem.min(usize::MAX as u128) as usize;

    (lb, None)
  }
}

impl DoubleEndedIterator for IpRangeIter {
  fn next_back(&mut self) -> Option<Self::Item> {
    if self.done {
      return None;
    }

    let out = self.end;
    if self.cur == self.end {
      self.done = true;
    } else {
      self.end = self.end.wrapping_sub(1);
    }

    Some(std::net::Ipv4Addr::from(out))
  }
}

impl FusedIterator for IpRangeIter {}

impl IntoIterator for IpRange {
  // iterate by value, ex. for ip in ip_range { ... }
  type Item = std::net::Ipv4Addr;
  type IntoIter = IpRangeIter;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl IntoIterator for &IpRange {
  // iterate by reference, ex. for ip in &ip_range { ... }
  type Item = std::net::Ipv4Addr;
  type IntoIter = IpRangeIter;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}


// ===== grammar test helpers =====


pub fn matches_octet(s: &str) -> bool { full_match(Rule::octet, s) }
pub fn matches_ip(s: &str) -> bool { full_match(Rule::ip, s) }
pub fn matches_cidr_prefix(s: &str) -> bool { full_match(Rule::cidr_prefix, s) }
pub fn matches_range_cidr(s: &str) -> bool { full_match(Rule::range_cidr, s) }
pub fn matches_range_hyphen(s: &str) -> bool { full_match(Rule::range_hyphen, s) }
pub fn matches_range_wildcard(s: &str) -> bool { full_match(Rule::range_wildcard, s) }
pub fn matches_range_any(s: &str) -> bool { full_match(Rule::range_any, s) }


// ===== core logic helpers =====


fn full_match(rule: Rule, input: &str) -> bool {
  match IpParser::parse(rule, input) {
    Ok(mut pairs) => {
      let p = pairs.next().unwrap();
      p.as_str() == input
    }

    Err(_) => false,
  }
}

fn parse_ipv4(s: &str) -> Result<u32, IpRangeError> {
  // convert quad octet string to u32
  let parts: Vec<&str> = s.split('.').collect();

  if parts.len() != 4 {
    return Err(IpRangeError::InvalidIp(s.to_string()));
  }

  let mut acc: u32 = 0;
  for (i, p) in parts.iter().enumerate() {
    let oct = p.parse::<u8>().map_err(|_| IpRangeError::InvalidIp(s.to_string()))?;
    acc |= (oct as u32) << (8 * (3 - i as u32));
  }

  Ok(acc)
}

fn mask_from_prefix(prefix: u8) -> u32 {
  // prefix in 0..=32
  if prefix == 0 { 0 }
  else { u32::MAX << (32 - prefix) }
}

fn size_from_prefix(prefix: u8) -> u128 {
  // 2^(32-prefix)
  1u128 << (32 - prefix)
}

fn infer_cidr_from_range(start: u32, end: u32) -> Option<u8> {
  let size = (end as u128).wrapping_sub(start as u128) + 1;

  if size == 0 { return None; }
  if (size & (size - 1)) != 0 { return None; }

  let size_u32 = if size > (u32::MAX as u128) + 1 {
    return None;
  } else { size as u32 };

  if start & (size_u32 - 1) != 0 {
    return None;
  }

  let trailing = size_u32.trailing_zeros();
  let prefix = 32 - trailing as u8;
  Some(prefix)
}


// ===== type conversions =====


impl FromStr for IpRange {
  type Err = IpRangeError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    IpRange::parse(s)
  }
}
