# web-sys-ec

Expected conditions in Selenium-like style for WASM targets using [`web-sys`].

## Rationale

When you test your apps with `wasm-pack test`, you may want to use the
[`web-sys`] crate to interact with the DOM. This is a great way to test your
app in a real browser environment.

However, a lot of actions in `web-sys` are asynchronous, and you may want to
wait for certain conditions to be met before proceeding with your tests. This
is where the `web-sys-ec` crate comes in.

It provides a library to interact with the DOM and wait for certain conditions
to be met. It is inspired by the Selenium syntax, where you can wait for
certain expected conditions to be met before proceeding with your tests.

## Disclaimer

I've created this library to help me with my own testing needs and it's far
away from being complete. Feel free to open pull requests or issues if you
find any bugs or have suggestions for improvements and I'll be happy to
review them.

## Installation

Add `web-sys-ec` to your `Cargo.toml`:

```toml
[dependencies]
web-sys-ec = "0.1"
```

## Example

```rust
use web_sys_ec::{By, Ec, Wait};

// wait 10 seconds for a P element to contain the text "Select a language:"
// or panic if it times out
Wait(10).until((
    By::TagName("p".into()),
    Ec::InnerTextContains("Select a language:".into()),
)).await;
```

[`web-sys`]: https://crates.io/crates/web-sys
