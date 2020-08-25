import {
    Board,
    Cell,
} from "watch";
import {
    memory
} from "watch/watch_bg";

/// Drawn cell size.
const CELL_SIZE = 5; // [px]
/// Grid colour.
const GRID_COL = "#CCCCCC";
/// Dead cell colour.
const DEAD_COL = "#FFFFFF";
/// Living cell colour.
const ALIVE_COL = "#000000";

/// Main board.
const board = Board.new(64, 64);
/// Board width [cells].
const width = board.width();
/// Board height [cells].
const height = board.height();

/// Canvas id.
const canvas = document.getElementById("main_canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
/// Drawing context.
const ctx = canvas.getContext('2d');

/// Rendering loop.
const renderLoop = () => {
    board.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
};

/// Draw the grid array.
const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COL;

    for (let j = 0; j <= height; ++j) {
        ctx.moveTo(0, (j * (CELL_SIZE + 1)) + 1);
        ctx.lineTo(((CELL_SIZE + 1) * width) + 1, (j * (CELL_SIZE + 1)) + 1);
    }
    for (let i = 0; i <= width; ++i) {
        ctx.moveTo((i * (CELL_SIZE + 1)) + 1, 0);
        ctx.lineTo((i * (CELL_SIZE + 1)) + 1, ((CELL_SIZE + 1) * height) + 1);
    }

    ctx.stroke();
};

/// Draw the cell array.
const drawCells = () => {
    const cellsPtr = board.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; ++row) {
        for (let col = 0; col < width; ++col) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead ?
                DEAD_COL :
                ALIVE_COL;

            ctx.fillRect(
                (col * (CELL_SIZE + 1)) + 1,
                (row * (CELL_SIZE + 1)) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

/// Get the one-dimensional index from the two-dimensional position.
const getIndex = (row, column) => {
    return row * width + column;
};

console.log("Hello world!");

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
