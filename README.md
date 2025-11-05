
# netdec

> (short for network deconstructor)

a tiny rust crate/cli for recognizing and enumerating common IPv4 range notations. this tool is a project for a university course so please temper your expectations.

in essence, this project is a collection of grammar rules and small scripts for the [`pest`](https://pest.rs) parser.



## installation

```bash
# in your project
cargo add netdec
```



## usage

### library

```rust
// ip range validation

use netdec::is_valid_ip_range;

assert!(is_valid_ip_range("10.*")) // True
assert!(is_valid_ip_range("192.168.0.0/24")) // True
assert!(is_valid_ip_range("256.0.0.1")) // False
```

```rust
OTHER FEATURES TBA
```

### cli

```bash
TBA
```



## planned features

- subnet evaluation
> given any supported range, determine the equivalent subnet mask and network/broadcast where possible.
- enumeration
>enumerate every IPv4 address within the parsed range.



## tests

tests live under `tests/` and cover every supported rule

```bash
cargo test
```
