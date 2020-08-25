//! Universal state.

use crate::cell::Cell;
use ndarray::Array2;
use std::fmt::{Display, Formatter, Result};
use wasm_bindgen::prelude::*;

/// Game board.
#[wasm_bindgen]
pub struct Board {
    /// Board size.
    res: [usize; 2],
    /// Cell state array.
    cells: Array2<Cell>,
}

//  JS methods.
#[wasm_bindgen]
impl Board {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::new_sized(64, 64)
    }

    /// Iterate the board forward a single step.
    #[inline]
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.res[1] {
            for col in 0..self.res[0] {
                let index = [row as usize, col as usize];
                let cell = self.cells[index];
                let live_neighbors = self.num_neighbours(index);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[index] = next_cell;
            }
        }

        self.cells = next;
    }

    /// Render the board to a string.
    #[inline]
    #[must_use]
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// Retrieve the board width.
    #[inline]
    #[must_use]
    pub fn width(&self) -> u32 {
        self.res[0] as u32
    }

    /// Retrieve the board height.
    #[inline]
    #[must_use]
    pub fn height(&self) -> u32 {
        self.res[1] as u32
    }

    /// Reference the array of cells as a pointer.
    #[inline]
    #[must_use]
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl Board {
    /// Construct a new instance with a given board size.
    #[inline]
    #[must_use]
    fn new_sized(width: u32, height: u32) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);

        let total_cells = width * height;
        let cells: Vec<_> = (0..total_cells)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        let res = [width as usize, height as usize];
        let cells = Array2::from_shape_vec(res, cells).expect("Failed to construct array.");

        Self { res, cells }
    }

    /// Calculate the number of neighbours a given cell has.
    #[inline]
    #[must_use]
    fn num_neighbours(&self, index: [usize; 2]) -> u8 {
        let mut count = 0;
        for delta_row in [self.res[1] - 1, 0, 1].iter().cloned() {
            for delta_col in [self.res[0] - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                // Periodic.
                let row = (index[0] + delta_row) % self.res[1];
                let col = (index[1] + delta_col) % self.res[0];

                count += self.cells[[row, col]] as u8;
            }
        }
        count
    }
}

impl Display for Board {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        for line in self
            .cells
            .as_slice()
            .expect("Failed to create slice from cell array.")
            .chunks(self.res[0] as usize)
        {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
