use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IpParser;

fn full_match(rule: Rule, input: &str) -> bool {
  match IpParser::parse(rule, input) {
    Ok(mut pairs) => {
      let p = pairs.next().unwrap();
      p.as_str() == input
    }
    
    Err(_) => false,
  }
}

// helpers, names are self explainatory
pub fn matches_octet(s: &str) -> bool { full_match(Rule::octet, s) }
pub fn matches_ip(s: &str) -> bool { full_match(Rule::ip, s) }
pub fn matches_cidr_suffix(s: &str) -> bool { full_match(Rule::cidr_suffix, s) }
pub fn matches_range_cidr(s: &str) -> bool { full_match(Rule::range_cidr, s) }
pub fn matches_range_hyphen(s: &str) -> bool { full_match(Rule::range_hyphen, s) }
pub fn matches_range_wildcard(s: &str) -> bool { full_match(Rule::range_wildcard, s) }
pub fn matches_range_any(s: &str) -> bool { full_match(Rule::range_any, s) }

pub fn is_valid_ip_range(input: &str) -> bool {
  matches_range_any(input)
}
