# jld-common

Common elements library for JSON Linked Data.

[![Made With Rust][made-with-rust-badge]][5] [![Crates.io][crates-badge]][7] [![Lib.rs][libs-badge]][9] [![Docs.rs][docs-badge]][8] [![License][license-badge]][2]

## Welcome to `jld-common` 👋

![jld-common banner][banner]

<!-- markdownlint-disable MD033 -->
<center>

**[Website][0]
• [Documentation][8]
• [Report Bug][3]
• [Request Feature][3]
• [Contributing Guidelines][4]**

</center>

<!-- markdownlint-enable MD033 -->

## Overview 📖

Common elements library for JSON Linked Data.

## Features ✨

- Feature 1
- Feature 2
- Feature 3

## Getting Started 🚀

It takes just a few minutes to get up and running with `jld-common`.

### Installation

To install `jld-common`, you need to have the Rust toolchain installed on
your machine. You can install the Rust toolchain by following the
instructions on the [Rust website][13].

Once you have the Rust toolchain installed, you can install `jld-common`
using the following command:

```shell
cargo install jld-common
```

You can then run the help command to see the available options:

```shell
jld-common --help
```

### Requirements

The minimum supported Rust toolchain version is currently Rust
**1.67.1** or later (stable).

### Platform support

`jld-common` is supported and tested on the following platforms:

#### Linux targets 🐧

| Target | Description | Status |
| --- | --- | --- |
| aarch64-unknown-linux-gnu | 64-bit Linux systems on ARM architecture | ✅ |
| aarch64-unknown-linux-musl | 64-bit Linux systems on ARM architecture | ✅ |
| arm-unknown-linux-gnueabi | ARMv6 Linux (kernel 3.2, glibc 2.17) | ✅ |
| armv7-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) | ✅ |
| i686-unknown-linux-gnu | 32-bit Linux (kernel 3.2+, glibc 2.17+) | ✅ |
| i686-unknown-linux-musl | 32-bit Linux (kernel 3.2+, musl libc) | ✅ |
| x86_64-unknown-linux-gnu | 64-bit Linux (kernel 2.6.32+, glibc 2.11+) | ✅ |
| x86_64-unknown-linux-musl | 64-bit Linux (kernel 2.6.32+, musl libc) | ✅ |

#### macOS targets 🍎

| Target | Description | Status |
| --- | --- | --- |
| x86_64-apple-darwin | 64-bit macOS (10.7 Lion or later) | ✅ |

The [GitHub Actions][10] shows the platforms in which the `jld-common`
library tests are run.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information.
You can find our documentation on [docs.rs][8], [lib.rs][9] and
[crates.io][7].

## Usage 📖

To use the `jld-common` library in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
jld-common = "0.1.0"
```

Add the following to your `main.rs` file:

```rust
extern crate jld-common;
use jld-common::*;
```

then you can use the functions in your application code.

### Examples

To get started with `jld-common`, you can use the examples provided in the
`examples` directory of the project.

To run the examples, clone the repository and run the following command
in your terminal from the project root directory.

```shell
cargo run --example jld-common
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `jld-common` follows [semantic versioning][6].

## License 📝

The project is licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

- [Apache License, Version 2.0][1]
- [MIT license][2]

## Contribution 🤝

We welcome all people who want to contribute. Please see the
[contributing instructions][4] for more information.

Contributions in any form (issues, pull requests, etc.) to this project
must adhere to the [Rust's Code of Conduct][11].

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Acknowledgements 💙

A big thank you to all the awesome contributors of [jld-common][5] for their
help and support.

A special thank you goes to the [Rust Reddit][12] community for
providing a lot of useful suggestions on how to improve this project.

[0]: https://test.com
[1]: https://opensource.org/license/apache-2-0/
[2]: http://opensource.org/licenses/MIT
[3]: https://github.com/test/test/jld-common/issues
[4]: https://github.com/test/test/jld-common/blob/main/CONTRIBUTING.md
[5]: https://github.com/test/test/jld-common/graphs/contributors
[6]: http://semver.org/
[7]: https://crates.io/crates/jld-common
[8]: https://docs.rs/jld-common
[9]: https://lib.rs/crates/jld-common
[10]: https://github.com/test/test/jld-common/actions
[11]: https://www.rust-lang.org/policies/code-of-conduct
[12]: https://www.reddit.com/r/rust/
[13]: https://www.rust-lang.org/learn/get-started

[banner]: https://via.placeholder.com/1500x500.png/000000/FFFFFF?text=jld-common "jld-common banner"
[crates-badge]: https://img.shields.io/crates/v/jld-common.svg?style=for-the-badge 'Crates.io badge'
[docs-badge]: https://img.shields.io/docsrs/jld-common.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.1.0-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/jld-common.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'
