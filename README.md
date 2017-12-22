# timmy-rs

Timmy is a timing and data library in rust. It provides a way to attach a
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
timmy = { git = "https://github.com/twh2898/timmy-rs.git" }
```

main.rs
```rust
extern crate timmy;
use timmy::Timed;

fn main() {
	let timed_int = Timed(3);
	println!("Data {} was created at time {}", timed_int.data(), timed_int.timestamp());
}
```

## Licence

timmy-rs uses the [MIT](LICENCE) licence
