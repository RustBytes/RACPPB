# RACPPB

Rust and C Plus Plus binding.

Create a new example from this with Cargo Generate. e.g.
```sh
cargo generate --git https://forge.rustbytes.uk/RustBytes/RACPPB
```

Some default tests are wrote to test that the basic functionality of the C++ integer is working with Rust.

Everything is also public so it can be used in another project like so:
```toml
[dependencies]
racppb = { path = "../RACPPB/" }
```
