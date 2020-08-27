/* tslint:disable */
/* eslint-disable */
/**
* Game board.
*/
export class Board {
  free(): void;
/**
* Construct a new instance.
* @param {number} width
* @param {number} height
* @returns {Board}
*/
  static new(width: number, height: number): Board;
/**
* Iterate the board forward a single step.
*/
  tick(): void;
/**
* Retrieve the board width.
* @returns {number}
*/
  width(): number;
/**
* Retrieve the board height.
* @returns {number}
*/
  height(): number;
/**
* Reference the array of cells as a pointer.
* @returns {number}
*/
  cells(): number;
/**
* Randomise the status of a cell.
* @param {number} x
*/
  randomise(x: number): void;
/**
* Toggle the status of a cell.
* @param {number} row
* @param {number} col
*/
  toggle_cell(row: number, col: number): void;
/**
* Kill all the cells.
*/
  nuke(): void;
}
