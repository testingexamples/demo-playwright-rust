---
name: demo-playwright-rust-skill
description: Use when asked to run, explain, or extend the demo-playwright-rust locator-strategy walkthrough, or to build a similar Playwright demo in Rust against a different site.
---

# Demo Playwright Rust Skill

This repo teaches five Playwright locator strategies and four form
interactions against the public page https://testingexamples.github.io,
using the `playwright-rs` crate (`padamson/playwright-rust`) — actively
maintained but still pre-1.0. Do not confuse it with the older, abandoned
`playwright` crate (`octaltree/playwright-rust`).

Locator strategies: by id (`#id-example-1`), by name attribute
(`[name="name-example-1"]`), by class (`.class-example-1`), by link text
(`a:has-text("Link Example 1")`), and by XPath
(`xpath=//input[@type="submit"]`).

Form interactions: fill a text input (`#text-example-1-id` with `"hello"`),
check a checkbox (`#checkbox-example-1-id`), check a radio button
(`#radio-example-1-option-1-id`), and select an option in a `<select>`
(`#select-example-1-id`, by index 0).

Each step prints the located element's text content or value — this is a
walkthrough that demonstrates locators, not a test suite with assertions.

## Running it

Install Rust/Cargo per README.md, then run `cargo run`. A visible
(non-headless) Chromium window opens, walks the page, and closes.

## Adapting it to a different site or selectors

1. Update the target URL passed to `page.goto(...)`.
2. Update each locator string to match the new page's real ids/classes/attributes/link text/XPath.
3. Update the values used in `.fill(...)`, `.check(...)`, and `.select_option(...)` to match the new form's real inputs.
4. Update `spec/index.md` to describe the new scenario verbatim, in the same change — not after.

This skill summarizes the repo. `AGENTS.md` and `spec/index.md` are the
source of truth — if this skill's summary ever disagrees with those, they
win.
