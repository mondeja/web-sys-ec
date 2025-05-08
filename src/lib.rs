#![cfg_attr(feature = "nightly", feature(async_fn_track_caller))]

//! [![Crates.io](https://img.shields.io/crates/v/web-sys-ec?logo=rust)](https://crates.io/crates/web-sys-ec)
//! [![License](https://img.shields.io/crates/l/web-sys-ec?logo=mit)](https://github.com/mondeja/web-sys-ec/blob/master/LICENSE)
//! [![Tests](https://img.shields.io/github/actions/workflow/status/mondeja/web-sys-ec/ci.yml?label=tests&logo=github)](https://github.com/mondeja/web-sys-ec/actions)
//! [![docs.rs](https://img.shields.io/docsrs/web-sys-ec?logo=docs.rs)](https://docs.rs/web-sys-ec)
//! [![Crates.io downloads](https://img.shields.io/crates/d/web-sys-ec)](https://crates.io/crates/web-sys-ec)
//!
//! Expected conditions in Selenium-like style for WASM targets using [`web-sys`].
//!
//! # Rationale
//!
//! When you test your apps with `wasm-pack test`, you may want to use the
//! [`web-sys`] crate to interact with the DOM. This is a great way to test your
//! app in a real browser environment.
//!
//! However, actions in a browser are asynchronous, and you may want to wait for
//! certain conditions to be met while testing. This is where `web-sys-ec` comes
//! in.
//!
//! Provides an interface to interact with the DOM waiting for certain conditions
//! to be met. It is inspired by the Selenium syntax, where you can wait for
//! certain expected conditions to be met before proceeding with your tests.
//!
//! # Disclaimer
//!
//! I've created this library to help me with my own testing needs and it's far
//! away from being complete. Feel free to open pull requests or issues if you
//! find any bugs or have suggestions for improvements and I'll be happy to
//! review them.
//!
//! # Installation
//!
//! Add `web-sys-ec` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! web-sys-ec = "0.1"
//! ```
//!
//! # Usage
//!
//! Wait 10 seconds for a `<p>` HTML element to contain the text `"Select a language:"`:
//!
//! ```rust,ignore
//! use web_sys_ec::{By, Ec, Wait};
//!
//! Wait(10)
//!     .until((
//!         By::TagName("p"),
//!         Ec::InnerTextContains("Select a language:"),
//!     ))
//!     .await;
//! ```
//!
//! Wait 1 second for the `<html>` HTML element to have the `lang` attribute set to
//! `"es"`:
//!
//! ```rust,ignore
//! use web_sys_ec::{By, Ec, Wait};
//!
//! Wait(1)
//!     .until((
//!         By::TagName("html"),
//!         Ec::AttributeValueIs("lang", "es"),
//!     ))
//!     .await;
//! ```
//!
//! Wait 1 second for the local storage to have the `language` key set to `"es"`:
//!
//! ```rust,ignore
//! use web_sys_ec::{Ec, Wait};
//!
//! Wait(1).until(Ec::LocalStorageAttributeValueIs("language","es")).await;
//! ```
//!
//! Wait 200 milliseconds for a `<p id="foo">` HTML element to exist in the DOM:
//!
//! ```rust,ignore
//! use web_sys_ec::{By, Wait};
//!
//! Wait(0.2).until("p#foo").await;
//! ```
//!
//! If a condition is not met, it will panic with a message like:
//!
//! <!-- markdownlint-disable MD013 -->
//!
//! ```txt
//!     Expected condition has not been met in the given time:
//!       - Caller: tests/end2end/tests/csr_complete.rs:54:10
//!       - Selector: HTML element with tag name 'html' (`By::TagName("html")`)
//!       - Condition: HTML element attribute 'lang' value is equal to 'es' (`Ec::AttributeValueIs("lang", "es")`)
//!       - Duration: 1s
//!       - Poll frecuency: 20ms
//!       - Number of attempts: 51
//! ```
//!
//! <!-- markdownlint-enable MD013 -->
//!
//! Note that the `Caller: ...` line will only be shown if you're using the `nightly`
//! toolchain and the `nightly` feature is enabled.
//!
//! # Features
//!
//! - `nightly`: Enables nightly toolchain support, which is currently needed to
//!   provide caller tracking.
//!
//! [`web-sys`]: https://crates.io/crates/web-sys

pub(crate) mod by;
mod condition;
pub(crate) mod ec;
mod until;
mod wait;
mod wait_options;

pub use by::By;
pub(crate) use condition::Condition;
pub use ec::Ec;
pub(crate) use until::{until_impl, until_not_impl};
pub use wait::Wait;
#[doc(hidden)]
pub(crate) use wait::Wait as Waiter;
pub use wait_options::WaitOptions;
