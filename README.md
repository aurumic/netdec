
# netdec

> (short for network deconstructor)

a tiny rust crate/cli for recognizing and enumerating common IPv4 range notations. this tool is a project for a university course so please temper your expectations.

in essence, this project is a collection of grammar rules and small scripts for the [`pest`](https://pest.rs) parser.



## installation

> note that this crate is not published yet. it will be once it's atleast semi-complete

```bash
# in your project
cargo add netdec
```



## usage

### library

```rust
// IpRange struct containing information about the parsed ip range
use netdec::IpRange;
use std::net::Ipv4Addr;

let r: IpRange = "192.168.0.0/24".parse().unwrap();

// characteristics
assert_eq!(r.first, "192.168.0.0".parse::<Ipv4Addr>().unwrap());
assert_eq!(r.last,  "192.168.0.255".parse().unwrap());
assert_eq!(r.prefix, Some(24));
assert_eq!(r.netmask.unwrap(), "255.255.255.0".parse().unwrap());
assert_eq!(r.hostmask.unwrap(), "0.0.0.255".parse().unwrap());
assert_eq!(r.network.unwrap(), "192.168.0.0".parse().unwrap());
assert_eq!(r.broadcast.unwrap(), "192.168.0.255".parse().unwrap());
assert_eq!(r.size, 256);
assert!(r.contains("192.168.0.42".parse().unwrap()));
```

```rust
// iterate by reference
for ip in r.iter().take(3) {
  println!("{ip}"); // 192.168.0.0, 192.168.0.1, 192.168.0.2
}

// reverse iteration
for ip in r.iter().rev().take(2) {
  // 192.168.0.255, 192.168.0.254
}
```

> ITERATING OVER LARGE ADDRESS POOLS (like those provided by *) IS NOT RECOMMENDED. USE WITH DISCRETION!

```rust
// other supported notations
let h: IpRange = "10.0.0.9-10.0.0.12".parse().unwrap();
let w: IpRange = "10.*".parse().unwrap();
```

### cli

```bash
# usage
netdec <RANGE>

# examples
netdec 192.168.0.0/24
netdec 192.168.0.1-192.168.0.10
netdec '10.*'

# sample output
input: 192.168.0.0/24
kind: cidr (base=192.168.0.0, /24)
first: 192.168.0.0
last:  192.168.0.255
size:  256
prefix: 24
netmask: 255.255.255.0
hostmask: 0.0.0.255
network: 192.168.0.0
broadcast: 192.168.0.255
```



## planned features

- [x] subnet evaluation
> given any supported range, determine the equivalent subnet mask and network/broadcast where possible.
- [x] enumeration
> enumerate every IPv4 address within the parsed range.



## tests

tests live under `tests/` and cover every supported rule and function

```bash
cargo test
```
