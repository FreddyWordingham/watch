//! Cell properties.

use wasm_bindgen::prelude::*;

/// Cell status enumeration.
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    /// Dead state.
    Dead = 0,
    /// Living state.
    Alive = 1,
}
