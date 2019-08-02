<h1 align="center">Runtime</h1>
<div align="center">
 <strong>
   Empowering everyone to build asynchronous software.
 </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/runtime">
    <img src="https://img.shields.io/crates/v/runtime.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Build Status -->
  <a href="https://dev.azure.com/yoshuawuyts/rustasync/_build?definitionId=2">
    <img src="https://img.shields.io/azure-devops/build/yoshuawuyts/rustasync/2/master.svg?style=flat-square"
      alt="Build Status" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/runtime">
    <img src="https://img.shields.io/crates/d/runtime.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/runtime">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/runtime">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/rustasync/runtime/blob/master/.github/CONTRIBUTING.md">
      Contributing
    </a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/474974025454452766">
      Chat
    </a>
  </h3>
</div>

<div align="center">
  <sub>Built with ⛵ by <a href="https://github.com/rustasync">The Rust Async Ecosystem WG</a>
</div>

## About
Runtime is what we imagine async APIs could look like if they were part of stdlib. We want async
Rust to be an experience that mirrors the quality of the standard lib. We believe that in order for
Rust to succeed it's not only important to make async Rust _possible_, it's crucial to make async
Rust feel _seamless_.

And the embodiment of these values is Runtime: a library crafted to empower everyone to build
asynchronous software.

- __runtime agnostic:__ Runtime comes with minimal OS bindings out of the box, but switching to a
    different runtime is a matter of changing a single line.
- __await anywhere:__ Runtime allows you to write async main functions, async tests, and async
    benchmarks. Experience what first-class async support in Rust feels like.
- __built for performance:__ Runtime is the thinnest layer possible on top of the backing
    implementations. All of the speed, none of the boilerplate.

## Examples
__UDP Echo Server__
```rust
#![feature(async_await)]

use runtime::net::UdpSocket;

#[runtime::main]
async fn main() -> std::io::Result<()> {
    let mut socket = UdpSocket::bind("127.0.0.1:8080")?;
    let mut buf = vec![0u8; 1024];

    println!("Listening on {}", socket.local_addr()?);

    loop {
        let (recv, peer) = socket.recv_from(&mut buf).await?;
        let sent = socket.send_to(&buf[..recv], &peer).await?;
        println!("Sent {} out of {} bytes to {}", sent, recv, peer);
    }
}
```

To send messages do:
```sh
$ nc -u localhost 8080
```

__More Examples__
- [Hello World](https://github.com/rustasync/runtime/tree/master/examples/hello.rs)
- [Guessing Game](https://github.com/rustasync/runtime/blob/master/examples/guessing.rs)
- [TCP Echo Server](https://github.com/rustasync/runtime/blob/master/examples/tcp-echo.rs)
- [TCP Client](https://github.com/rustasync/runtime/tree/master/examples/tcp-client.rs)
- [TCP Proxy Server](https://github.com/rustasync/runtime/tree/master/examples/tcp-proxy.rs)
- [UDP Echo Server](https://github.com/rustasync/runtime/tree/master/examples/udp-echo.rs)
- [UDP Client](https://github.com/rustasync/runtime/tree/master/examples/udp-client.rs)

## Attributes
Runtime introduces 3 attributes to enable the use of await anywhere, and swap between different
runtimes. Each Runtime is bound locally to the initializing thread. This enables the testing of
different runtimes during testing or benchmarking.

```rust
#[runtime::main]
async fn main() {}

#[runtime::test]
async fn my_test() {}

#[runtime::bench]
async fn my_bench() {}
```

## Runtimes
Switching runtimes is a one-line change:

```rust
/// Use the default Native Runtime
#[runtime::main]
async fn main() {}

/// Use the Tokio Runtime
#[runtime::main(runtime_tokio::Tokio)]
async fn main() {}
```

The following backing runtimes are available:

- [Runtime Native (default)](https://crates.io/crates/runtime-native) provides
  a thread pool, bindings to the OS, and a concurrent scheduler.
- [Runtime Tokio](https://crates.io/crates/runtime-tokio) provides a thread pool, bindings to the OS, and
  a work-stealing scheduler.

## Performance
Runtime provides performance that's competitive with most other systems languages, and great
ergonomics to match.

Because we don't know what your workload is like, we can't predict which runtime will be able to
maximize resource consumption for your use case.

But we can tell from our benchmarks that the difference between using Runtime and not using Runtime
doesn't show up for IO-bound applications.
```txt
 name          baseline:: ns/iter  native:: ns/iter  diff ns/iter    diff %  speedup
 notify_self   1,350,882           1,237,416             -113,466    -8.40%   x 1.09
 poll_reactor  2,270,428           2,162,264             -108,164    -4.76%   x 1.05
```

## Installation
With [cargo-edit](https://crates.io/crates/cargo-edit) do:
```sh
$ cargo add runtime --allow-prerelease
```

To use Futures in the same project, make sure to install
[futures-preview](https://docs.rs/futures-preview/) for std futures.
```sh
$ cargo add futures-preview --allow-prerelease
```

`futures-preview` provides support for std futures/futures 0.3, while `futures` provides support for
the no longer developed futures 0.1. Once futures land in stdlib, it's expected that the two crates
will merge back into `futures`. With the hopes that eventually most of `futures` will be part of
stdlib.

## FAQ
### When is it useful to switch Runtimes?
What might be the best solution now, might not stay the best in the future. As Rust grows, so will
the ecosystem. By making runtimes pluggable, your code can be forward compatible with any future
changes. And as things evolve, you'll be able to test out the benefit new developments in the
ecosystem have on your code by just changing a single line.

### How is Runtime versioned?
We're currently in the `0.3-alpha` range of releases, mirroring the Futures libraries. Once
Futures hits 1.0, we'll follow suit and move over to semver proper.

This doesn't mean that Runtime won't release breaking changes. But if we do, we'll release a new
major version, and provide instructions on how to upgrade. We view Runtime to be a foundational
piece of technology, and that means we have to be serious about our stability guarantees.

### Can I use Runtime in production?
Runtime is a thin layer that sits between your code and the backing runtimes. If you trust the
backing runtime in production, then you can probably trust Runtime too.

### Why is Runtime Native the default?
We believe Runtime Native provides a balanced implementation that works well for most scenarios. The
codebase is small and comprehensive, and the algorithms simple yet performant.

Specific runtimes might introduce different trade-offs, and with Runtime you're able to compare, and
pick the best fit for your requirements.

### Can Runtime be used on embedded devices?
Runtime is designed to be compatible with micro processors, but not with micro controllers. Out of
the box Runtime works on embedded devices such as Raspberry Pis, and with the appropriate backends
it should also work on phones.

Micro controllers are very specific in what they provide, and while a Runtime-like library might be
possible in the future, it's still early for the ecosystem and APIs would likely also need to be
different. We don't know what the future holds, but for now we've chosen not to target micro
controllers.

### When will Timers and File System support land?
Timers are next up on the list of things we want to target, together with Unix Domain Sockets.
Filesystem is a bit further behind because currently the implementations in the backing runtimes are
changing, and we're not sure yet how to best abstract that.

Getting things right takes time. But if you'd like to move the state of async forward, we'd love for
you to get involved!

## Safety
This crate uses `unsafe` in a few places to construct pin projections.

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

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[releases]: https://github.com/rustasync/runtime/releases
[contributing]: https://github.com/rustasync/runtime/blob/master/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/rustasync/runtime/labels/good%20first%20issue
[help-wanted]: https://github.com/rustasync/runtime/labels/help%20wanted
