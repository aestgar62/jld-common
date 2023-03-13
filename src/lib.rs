// Copyright © 2023 jld-common. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0
//!
//! # Common elements library for JSON Linked Data.
//!
//! [![jld-common](https://via.placeholder.com/1500x500.png/000000/FFFFFF?text=jld-common)](https://test.com)
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/jld-common.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/jld-common)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.1.0-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/jld-common)
//! [![License](https://img.shields.io/crates/l/jld-common.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](MIT OR Apache-2.0)
//!
//! ## Features
//!
//! - Serialization and deserialization of data structures to JSON format
//! - ...
//! - ...
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! jld-common = "0.1.0"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```
//!
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![doc(
    html_favicon_url = "",
    html_logo_url = "",
    html_root_url = "https://docs.rs/jld-common"
)]
#![crate_name = "jld_common"]
#![crate_type = "lib"]

mod error;
pub mod rdf;

pub use error::Error;