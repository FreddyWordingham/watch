//! Game of life binary test function.

use watch::*;

/// Main running function.
pub fn main() {
    let num_steps = 0;

    let mut board = Board::new();
    for _t in 0..num_steps {
        board.tick();
    }
    println!("{}", board);
}
