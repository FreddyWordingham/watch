//! Library core.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::else_if_without_else,
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::modulo_arithmetic,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::print_stdout,
    clippy::unreachable,
    clippy::unwrap_used
)]

pub mod life;

pub use self::life::*;

use wasm_bindgen::prelude::*;

/// Set the console error panic hook.
/// Required to be called once.
#[allow(dead_code)]
#[inline]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// Memory allocator override.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Console alert binding.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Test greeting function.
#[wasm_bindgen]
#[inline]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

