use std::env;
use netdec::IpRange;

fn main() {
  let Some(arg) = env::args().nth(1) else {
    eprintln!("usage: cargo run -- <INPUT>");
    std::process::exit(2);
  };

  if IpRange::is_valid(&arg) {
    println!("valid");
    std::process::exit(0);
  } else {
    println!("invalid");
    std::process::exit(1);
  }
}
