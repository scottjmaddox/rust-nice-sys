# rust-nice-sys

Rust FFI bindings to [libnice](https://nice.freedesktop.org/wiki/)

## How To Add a New Arch/OS Pair

To add support for a new architecture / operating system pair, follow the
directions below.

Install the `bindgen` executable with `cargo`:

```sh
cargo install bindgen
```

Generate bindings (replace `arch` with your `target_arch`, and `os` with your
`target_os`):

```sh
bindgen wrapper.h -o src/arch_os.rs -- `pkg-config --cflags nice`
```

Add a new entry in `lib.rs`(replace `arch` with your `target_arch`, and `os`
with your `target_os`):

```rust
#[cfg(all(target_arch = "arch", target_os = "os"))]
mod arch_os;
#[cfg(all(target_arch = "arch", target_os = "os"))]
pub use arch_os::*;
```

`target_arch` options:

- x86
- x86_64
- mips
- powerpc
- powerpc64
- arm
- aarch64

`target_os` options:

- windows
- macos
- ios
- linux
- android
- freebsd
- dragonfly
- bitrig
- openbsd
- netbsd

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
