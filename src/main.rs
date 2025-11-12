use netdec::{IpRange, RangeKind};
use std::env;
use std::process;

fn print_usage() {
  eprintln!("
usage:
  {bin} <RANGE>

examples:
  {bin} 192.168.0.0/24
  {bin} 192.168.0.1-192.168.0.10
  {bin} 10.*

notes:
  in shells like bash/zsh/fish, '*' is a glob. quote or escape it
",
    bin = env!("CARGO_PKG_NAME")
  );
}

fn main() {
  let argv: Vec<String> = env::args().skip(1).collect();

  if argv.is_empty() {
    print_usage();
    process::exit(2);
  }

  if argv.len() == 1 && matches!(argv[0].as_str(), "-h" | "--help") {
    print_usage();
    process::exit(0);
  }

  if argv.len() > 1 {
    eprintln!("
error: received {} arguments
hint: quote the wildcard, e.g.:
  {bin} \"*\"
",
      argv.len(),
      bin = env!("CARGO_PKG_NAME")
    );
    process::exit(2);
  }

  let arg = &argv[0];
  match arg.parse::<IpRange>() {
    Ok(r) => {
      print_range(&r);
      process::exit(0);
    }
    
    Err(e) => {
      eprintln!("error: {e}");
      process::exit(2);
    }
  }
}

fn print_range(r: &IpRange) {
  println!("\ninput: {}", r.input);
  println!("kind: {}", kind_label(&r.kind));
  println!("first: {}", r.first);
  println!("last: {}", r.last);
  println!("size: {}", r.size);

  if let Some(p) = r.prefix {
    println!("prefix: {}", p);
  }
  if let Some(nm) = r.netmask {
    println!("netmask: {}", nm);
  }
  if let Some(hm) = r.hostmask {
    println!("hostmask: {}", hm);
  }
  if let Some(net) = r.network {
    println!("network: {}", net);
  }
  if let Some(bc) = r.broadcast {
    println!("broadcast: {}", bc);
  }

  println!()
}

fn kind_label(k: &RangeKind) -> String {
  match k {
    RangeKind::Cidr { base, prefix } => format!("cidr (base={base}, /{prefix})"),
    RangeKind::Hyphen { .. } => "hyphen".to_string(),
    RangeKind::Wildcard { specified_octets } => {
      format!("wildcard ({} octet{})",
        specified_octets,
        if *specified_octets == 1 { "" } else { "s" })
    }
  }
}
