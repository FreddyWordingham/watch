//! Universal state.

use fixedbitset::FixedBitSet;
use js_sys;
use std::fmt::{Display, Formatter, Result};
use wasm_bindgen::prelude::*;

// /// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

/// Game board.
#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Board {
    /// Board size.
    res: [u32; 2],
    /// Cell state array.
    cells: FixedBitSet,
    /// Previous state.
    prev: FixedBitSet,
}

//  JS methods.
#[wasm_bindgen]
#[allow(clippy::missing_const_for_fn)]
#[allow(clippy::missing_inline_in_public_items)]
impl Board {
    /// Construct a new instance.
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self::new_sized(width, height)
    }

    /// Iterate the board forward a single step.
    pub fn tick(&mut self) {
        // for _ in 0..100 {
        for row in 0..self.res[1] {
            for col in 0..self.res[0] {
                let index = self.get_index(row, col);
                let cell = self.cells[index];

                let live_neighbors = self.num_neighbours(row, col);

                self.prev.set(
                    index,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        std::mem::swap(&mut self.cells, &mut self.prev);
        // }
    }

    /// Retrieve the board width.
    #[must_use]
    pub fn width(&self) -> u32 {
        self.res[0] as u32
    }

    /// Retrieve the board height.
    #[must_use]
    pub fn height(&self) -> u32 {
        self.res[1] as u32
    }

    /// Reference the array of cells as a pointer.
    #[must_use]
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Randomise the status of a cell.
    pub fn randomise(&mut self, x: f64) {
        debug_assert!(x >= 0.0);
        debug_assert!(x <= 1.0);

        let total_cells = (self.res[0] * self.res[1]) as usize;
        for i in 0..total_cells {
            let status = js_sys::Math::random() < x;
            self.cells.set(i, status);
        }
    }

    /// Toggle the status of a cell.
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        self.cells.toggle(self.get_index(row, col));
    }

    /// Kill all the cells.
    pub fn nuke(&mut self) {
        self.cells.set_range(0..self.cells.len(), false);
    }
}

impl Board {
    /// Construct a new instance with a given board size.
    #[inline]
    #[must_use]
    fn new_sized(width: u32, height: u32) -> Self {
        crate::set_panic_hook(); // TODO: Better place for this?

        debug_assert!(width > 0);
        debug_assert!(height > 0);

        let total_cells = (width * height) as usize;

        Self {
            res: [width, height],
            cells: FixedBitSet::with_capacity(total_cells),
            prev: FixedBitSet::with_capacity(total_cells),
        }
    }

    /// Get the linear index corresponding to the two-dimensional spatial position.
    #[inline]
    #[must_use]
    const fn get_index(&self, row: u32, col: u32) -> usize {
        ((row * self.res[0]) + col) as usize
    }

    // /// Calculate the number of neighbours a given cell has.
    // #[inline]
    // #[must_use]
    // fn num_neighbours(&self, row: u32, col: u32) -> u8 {
    //     let mut count = 0;
    //     for delta_row in [self.res[1] - 1, 0, 1].iter().cloned() {
    //         for delta_col in [self.res[0] - 1, 0, 1].iter().cloned() {
    //             if delta_row == 0 && delta_col == 0 {
    //                 continue;
    //             }

    //             // Periodic.
    //             let r = (row + delta_row) % self.res[1];
    //             let c = (col + delta_col) % self.res[0];

    //             count += self.cells[self.get_index(r, c)] as u8;
    //         }
    //     }
    //     count
    // }

    /// Calculate the number of neighbours a given cell has.
    #[inline]
    #[must_use]
    fn num_neighbours(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.res[1] - 1 } else { row - 1 };

        let south = if row == self.res[1] - 1 { 0 } else { row + 1 };

        let west = if col == 0 { self.res[0] - 1 } else { col - 1 };

        let east = if col == self.res[0] - 1 { 0 } else { col + 1 };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, col);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, col);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    /// Set a list of cells status' to alive.
    #[inline]
    pub fn make_alive(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            self.cells.set(self.get_index(row, col), true);
        }
    }
}

impl Default for Board {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::new(64, 64)
    }
}

impl Display for Board {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        for row in 0..self.res[1] {
            for col in 0..self.res[0] {
                let symbol = if self.cells[self.get_index(row, col)] {
                    '◼'
                } else {
                    '◻'
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
