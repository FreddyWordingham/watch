//! Cell status implementation.

/// Cell status.
pub enum Cell {
    /// Dead cell.
    Dead = 0,
    /// Living cell.
    Alive = 1,
}

impl Cell {
    /// Toggle the status of the cell.
    #[inline]
    pub fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}
