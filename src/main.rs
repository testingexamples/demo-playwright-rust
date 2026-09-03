//! Demo of Playwright browser automation with Rust.
//!
//! Ported from the sibling `demo-playwright-javascript` walkthrough to the
//! `playwright-rs` crate. Please see the file README.md for more information.
//!
//! ## Tracking
//!
//!   * Package: demo-playwright-rust
//!   * Version: 1.0.0
//!   * Created: 2026-09-03T00:00:00Z
//!   * Updated: 2026-09-03T00:00:00Z
//!   * License: GPL-2.0-or-greater or for custom license contact us
//!   * Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)

use playwright_rs::{Page, Playwright, SelectOption};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Launch a browser and open a page.
    let pw = Playwright::launch().await?;
    let browser = pw.chromium().launch().await?;
    let page = browser.new_page().await?;

    // Run the walkthrough, but always close the browser afterward, even if
    // a step fails — mirroring the try/catch/finally shape of the
    // JavaScript and Python siblings.
    let result = run_demo(&page).await;

    if let Err(ref err) = result {
        eprintln!("{err:?}");
    }

    browser.close().await?;
    result
}

async fn run_demo(page: &Page) -> anyhow::Result<()> {
    // Navigate to the site.
    page.goto("https://testingexamples.github.io", None).await?;

    // ---
    // Find elements in various ways.
    // Note: Playwright locators auto-wait and auto-retry until an element
    // is actionable, so there is no explicit "wait for element" step.
    // ---

    // Find an element by id.
    //
    // This demonstrates a locator with an id selector.
    //
    // Example HTML:
    //
    //     <p id="id-example-1">Lorem Ipsum</p>
    //
    let element_by_id = page.locator("#id-example-1");
    println!("By id: {}", element_by_id.text_content().await?.unwrap_or_default());

    // Find an element by name attribute.
    //
    // This demonstrates a locator with an attribute selector.
    //
    // Example HTML:
    //
    //     <p name="name-example-1">Lorem Ipsum</p>
    //
    let element_by_name = page.locator("[name=\"name-example-1\"]");
    println!("By name: {}", element_by_name.text_content().await?.unwrap_or_default());

    // Find an element by class name.
    //
    // This demonstrates a locator with a CSS class selector.
    //
    // Example HTML:
    //
    //     <p class="class-example-1">Lorem Ipsum</p>
    //
    let element_by_class_name = page.locator(".class-example-1");
    println!(
        "By class name: {}",
        element_by_class_name.text_content().await?.unwrap_or_default()
    );

    // Find a link element by its visible text.
    //
    // This demonstrates a locator with a text-matching pseudo-class.
    //
    // Example HTML:
    //
    //     <a href="https://example.com">Link Example 1</a>
    //
    let element_by_link_text = page.locator("a:has-text(\"Link Example 1\")");
    println!(
        "By link text: {}",
        element_by_link_text.text_content().await?.unwrap_or_default()
    );

    // Find an element by an XPath expression.
    //
    // This demonstrates an XPath selector. The target is a submit input,
    // which carries its label in its `value` attribute rather than as text
    // content, so we read it with `.input_value()` instead.
    //
    // Example HTML:
    //
    //     <input type="submit">
    //
    let element_by_xpath = page.locator("xpath=//input[@type=\"submit\"]");
    println!("By XPath: {}", element_by_xpath.input_value(None).await?);

    // ---
    // Interact with form inputs in various ways.
    // ---

    // Fill in a text input.
    //
    // Example HTML:
    //
    //     <input type="text" id="text-example-1-id">
    //
    let text = page.locator("#text-example-1-id");
    text.fill("hello", None).await?;
    println!("Text input filled with: {}", text.input_value(None).await?);

    // Check a checkbox input.
    //
    // `.check()` mirrors the `check()` method every other Playwright
    // language binding exposes on its Locator type. If a future
    // `playwright-rs` release does not have it yet, `.click(None)` is the
    // confirmed-working fallback (a click also toggles a checkbox).
    //
    // Example HTML:
    //
    //     <input type="checkbox" id="checkbox-example-1-id">
    //
    let checkbox = page.locator("#checkbox-example-1-id");
    checkbox.check(None).await?;
    println!("Checkbox checked.");

    // Check a radio input.
    //
    // Example HTML:
    //
    //     <input type="radio" id="radio-example-1-option-1-id">
    //
    let radio = page.locator("#radio-example-1-option-1-id");
    radio.check(None).await?;
    println!("Radio button checked.");

    // Choose a select input option, by index.
    //
    // Example HTML:
    //
    //     <select id="select-example-1-id">
    //       <option>alfa</option>
    //       <option>bravo</option>
    //       <option>charlie</option>
    //     </select>
    //
    let select_element = page.locator("#select-example-1-id");
    select_element.select_option(SelectOption::Index(0), None).await?;
    let selected_value = select_element.input_value(None).await?;
    println!("Selected option value: {selected_value}");

    Ok(())
}
