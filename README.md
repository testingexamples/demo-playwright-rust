# Demo Playwright Rust

Demonstration of:

* [Playwright](https://www.playwright.dev/) browser automation testing
* [Rust](https://www.rust-lang.org/) programming language
* [Cargo](https://doc.rust-lang.org/cargo/) build tool and package manager
* [Chromium](https://www.chromium.org/) open source web browser

Playwright ships official bindings for JavaScript, Python, .NET, and Java.
Rust is community-maintained. This demo uses [`playwright-rs`](https://crates.io/crates/playwright-rs)
(`padamson/playwright-rust`), which is actively maintained but still
pre-1.0 and stabilising its API. Be careful which crate you install: an
older, unrelated crate published on crates.io simply as `playwright`
(`octaltree/playwright-rust`) has been abandoned since 2022 — don't reach
for that one.

The exact scenario this demo walks through (target URL, locators, form
interactions) is specified in [spec/index.md](spec/index.md); the code and
spec must agree.

## Install

### Install Rust and Cargo

Install Rust (which includes Cargo) from <https://www.rust-lang.org/tools/install>,
typically via `rustup`.

Run this to confirm your version:

```sh
rustc --version
```

Output should be at least:

```stdout
rustc 1.88.0
```

Run this to confirm your version:

```sh
cargo --version
```

### Install dependencies

```sh
cargo build
```

This downloads and compiles `playwright-rs` and the other dependencies
listed in [Cargo.toml](Cargo.toml).

### Install the Playwright browser binaries

`playwright-rs` needs Playwright's own browser binaries, the same way the
JavaScript and Python bindings do. Consult the crate's documentation for
the current install command (typically a `playwright install` style
step driven by the crate's own CLI helper).

### Update

Run:

```sh
cargo update
```

## Run

Run:

```sh
cargo run
```

The script will do three things:

1. Launch a local Chromium web browser to view the free open source testing
   examples web page <https://testingexamples.github.io>.

2. Interact with the web page in various ways, such as finding elements,
   clicking on elements, filling in form inputs, etc.

3. Print some typical output that demonstrates the program is running
   successfully.

## Tracking

* Package: demo-playwright-rust
* Version: 1.0.0
* Created: 2026-09-03T00:00:00Z
* Updated: 2026-09-03T00:00:00Z
* License: GPL-2.0-or-greater or for custom license contact us
* Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)
