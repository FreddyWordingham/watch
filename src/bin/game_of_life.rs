//! Game of life binary test function.

use watch::*;

/// Main running function.
pub fn main() {
    let num_steps = 1000;

    let mut board = Board::new();
    for t in 0..num_steps {
        board.tick();
    }
    println!("{}", board);
}
