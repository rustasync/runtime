# runtime-attributes
Proc Macro attributes for the [Runtime](https://github.com/rustasync/runtime) crate. See the
[Runtime](https://docs.rs/runtime) documentation for more details.

__This macro was designed to be used from the Runtime crate. Using this in any other way is unlikely
to work.__

## Installation
With [cargo-edit](https://crates.io/crates/cargo-edit) do:
```sh
$ cargo add runtime-attributes
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in 100% Safe Rust.

## Contributing
Want to join us? Check out our [The "Contributing" section of the
guide][contributing] and take a look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

#### Conduct

The Runtime project adheres to the [Contributor Covenant Code of
Conduct](https://github.com/rustasync/runtime/blob/master/.github/CODE_OF_CONDUCT.md).  This
describes the minimum behavior expected from all contributors.

## License
Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[releases]: https://github.com/rustasync/runtime/releases
[contributing]: https://github.com/rustasync/runtime/blob/master/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/rustasync/runtime/labels/good%20first%20issue
[help-wanted]: https://github.com/rustasync/runtime/labels/help%20wanted
