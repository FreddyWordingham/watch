//! Game of life binary test function.

use watch::*;

/// Main running function.
pub fn main() {
    let num_steps = 3;

    let mut board = Board::new(64, 64);
    for t in 0..num_steps {
        board.tick();
        println!("{}:", t);
        println!("{}", board);
    }
}
