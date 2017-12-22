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

Cargo.toml
```toml
[dependencies]
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

## Licence

timy-rs uses the [MIT](LICENCE) licence
