//! Universal state.

use fixedbitset::FixedBitSet;
use std::fmt::{Display, Formatter, Result};
use wasm_bindgen::prelude::*;

/// Game board.
#[wasm_bindgen]
pub struct Board {
    /// Board size.
    res: [u32; 2],
    /// Cell state array.
    cells: FixedBitSet,
}

//  JS methods.
#[wasm_bindgen]
impl Board {
    /// Construct a new instance.
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self::new_sized(width, height)
    }

    /// Iterate the board forward a single step.
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.res[1] {
            for col in 0..self.res[0] {
                let index = self.get_index(row, col);
                let cell = self.cells[index];

                let live_neighbors = self.num_neighbours(row, col);

                next.set(
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

        self.cells = next;
    }

    /// Render the board to a string.
    #[must_use]
    pub fn render(&self) -> String {
        self.to_string()
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
}

impl Board {
    /// Construct a new instance with a given board size.
    #[inline]
    #[must_use]
    fn new_sized(width: u32, height: u32) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);

        let total_cells = (width * height) as usize;

        let mut cells = FixedBitSet::with_capacity(total_cells);
        for i in 0..total_cells {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Self {
            res: [width, height],
            cells,
        }
    }

    /// Get the linear index corresponding to the two-dimensional spatial position.
    #[inline]
    #[must_use]
    const fn get_index(&self, row: u32, col: u32) -> usize {
        ((row * self.res[0]) + col) as usize
    }

    /// Calculate the number of neighbours a given cell has.
    #[inline]
    #[must_use]
    fn num_neighbours(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.res[1] - 1, 0, 1].iter().cloned() {
            for delta_col in [self.res[0] - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                // Periodic.
                let r = (row + delta_row) % self.res[1];
                let c = (col + delta_col) % self.res[0];

                count += self.cells[self.get_index(r, c)] as u8;
            }
        }
        count
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
