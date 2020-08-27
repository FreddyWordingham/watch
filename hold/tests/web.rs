use wasm_bindgen_test::*;
use watch::Board;

#[cfg(test)]
pub fn input_spaceship() -> Board {
    let mut board = Board::new(6, 6);
    board.make_alive(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    board
}

#[cfg(test)]
pub fn expected_spaceship() -> Board {
    let mut board = Board::new(6, 6);
    board.make_alive(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    board
}

#[wasm_bindgen_test]
pub fn test_tick() {
    let mut input_universe = input_spaceship();
    input_universe.tick();

    let expected_universe = expected_spaceship();

    assert_eq!(&input_universe, &expected_universe);
}
