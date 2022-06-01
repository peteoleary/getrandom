# getrandom

[![Build Status]][GitHub Actions] [![Crate]][crates.io] [![Documentation]][docs.rs] [![Dependency Status]][deps.rs] [![Downloads]][crates.io] [![License]][LICENSE-MIT]

[GitHub Actions]: https://github.com/rust-random/getrandom/actions?query=workflow:Tests+branch:master
[Build Status]: https://github.com/rust-random/getrandom/workflows/Tests/badge.svg?branch=master
[crates.io]: https://crates.io/crates/getrandom
[Crate]: https://img.shields.io/crates/v/getrandom
[docs.rs]: https://docs.rs/getrandom
[Documentation]: https://docs.rs/getrandom/badge.svg
[deps.rs]: https://deps.rs/repo/github/rust-random/getrandom
[Dependency Status]: https://deps.rs/repo/github/rust-random/getrandom/status.svg
[Downloads]: https://img.shields.io/crates/d/getrandom
[LICENSE-MIT]: https://raw.githubusercontent.com/rust-random/getrandom/master/LICENSE-MIT
[License]: https://img.shields.io/crates/l/getrandom

## NEAR smart contract usuage

This fork introduces a `noop` (short for "no operation") feature which, when enabled and used with a build target of `wasm32-unknown-unknown` does... NOTHING! Well, not exactly. Actually, what it does is stub out the low level `getrandom_inner` function to panic when called with a helpful message to that effect. Why is this? Well, the only other way for the `getrandom` to compile into WASM is to set the `js` flag instead which makes the crate rely on browser or Node.js Javascript calls to implement `getrandom_inner`. However, this code https://github.com/peteoleary/getrandom/blob/4b15c004f4169ad8ce0a61cf428106a48a921f08/src/js.rs#L26 seems to get called when the crate is instantiated which blows up your NEAR smart contract with a very unhelpful error:

```
{
  "ActionError": {
    "index": 3,
    "kind": {
      "FunctionCallError": {
        "CompilationError": {
          "PrepareError": "Instantiate"
        }
      }
    }
  }
}
```

This means that if any crate you are pulling into your project relies on `rand` or `getrandom` it will blow up your contract. You will have to find a new crate to use or fork the crate and cut out the reliant code.

Instead, just path `getrandom` and turn on the `noop` feature in your `Cargo.toml` file:

```
[patch.crates-io]
getrandom = { git = "https://github.com/peteoleary/getrandom" }

[dependencies]
getrandom = { version="0.2.6", features = ["noop"] }
```

Your contract will (hopefully) instantiate without problem and your code won't blow up until it actually tries to get a random number at which point you will get an informative error message.

## Back to our regularly scheduled program (crate)

A Rust library for retrieving random data from (operating) system source. It is
assumed that system always provides high-quality cryptographically secure random
data, ideally backed by hardware entropy sources. This crate derives its name
from Linux's `getrandom` function, but is cross platform, roughly supporting
the same set of platforms as Rust's `std` lib.

This is a low-level API. Most users should prefer using high-level random-number
library like [`rand`].

[`rand`]: https://crates.io/crates/rand

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
getrandom = "0.2"
```

Then invoke the `getrandom` function:

```rust
fn get_random_buf() -> Result<[u8; 32], getrandom::Error> {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}
```

For more information about supported targets, entropy sources, `no_std` targets,
crate features, WASM support and Custom RNGs see the
[`getrandom` documentation](https://docs.rs/getrandom/latest) and
[`getrandom::Error` documentation](https://docs.rs/getrandom/latest/getrandom/struct.Error.html).

## Minimum Supported Rust Version

This crate requires Rust 1.34.0 or later.

# License

The `getrandom` library is distributed under either of

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

at your option.
