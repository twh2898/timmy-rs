# timy-rs

Timy is a timing and data library in rust. It provides a way to attach a
timestamp to any data in much the same way as an `Option` or `Result` type
works.

## Examples

```rust
let timed_int = Timed(3);
println!("Data {} was created at time {}", timed_int.data(), timed_int.timestamp());
```

## Usage

timy-rs depends on the [Chrono](https://crates.io/crates/chrono) crate for its
time stamps.

Cargo.toml
```toml
[dependencies]
chrono = "0.4"
timy = { git = "https://github.com/twh2898/timy-rs.git" }
```

main.rs
```rust
extern crate timy;
use timy::Timed;

fn main() {
	let timed_int = Timed(3);
	println!("Data {} was created at time {}", timed_int.data(), timed_int.timestamp());
}
```

## TODO

- [ ] Code Documentation
- [ ] Testing

## Licence

timy-rs uses the [MIT](LICENCE) licence
