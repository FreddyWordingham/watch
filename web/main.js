import {
    Board,
    Cell,
} from "watch";
import {
    memory
} from "watch/watch_bg";

/// Drawn cell size.
const CELL_SIZE = 10; // [px]
/// Grid colour.
const GRID_COL = "#CCCCCC";
/// Dead cell colour.
const DEAD_COL = "#FFFFFF";
/// Living cell colour.
const ALIVE_COL = "#000000";

/// Default board width.
const DEFAULT_WIDTH = 64;
/// Default board height.
const DEFAULT_HEIGHT = 64;

/// Main board.
const board = Board.new(DEFAULT_WIDTH, DEFAULT_HEIGHT);
board.randomise(0.367879441);
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

/// Form.
const wipe_button = document.getElementById("wipe_button");
const time_button = document.getElementById("time_toggle_button");



/// Check if time is paused.
const is_paused = () => {
    return frame_id === null;
};

/// Start time.
const play = () => {
    time_button.textContent = "stop";
    render_loop();
};

/// Stop time.
const pause = () => {
    time_button.textContent = "start";
    cancelAnimationFrame(frame_id);
    frame_id = null;
};

/// Check for button click.
time_button.addEventListener("click", event => {
    if (is_paused()) {
        play();
    } else {
        pause();
    }
});


/// Check for button click.
wipe_button.addEventListener("click", event => {
    board.nuke();
});

/// Check for keypress.
document.body.onkeyup = function (e) {
    if (e.keyCode == 32) { // Spacebar.
        if (is_paused()) {
            play();
        } else {
            pause();
        }
    }
}


/// Check for canvas clicking.
canvas.addEventListener("click", event => {
    console.log("Click!");

    const bound = canvas.getBoundingClientRect();

    const scale_x = canvas.width / bound.width;
    const scale_y = canvas.height / bound.height;

    const canvas_left = (event.clientX - bound.left) * scale_x;
    const canvas_top = (event.clientY - bound.top) * scale_y;

    const row = Math.min(Math.floor(canvas_top / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvas_left / (CELL_SIZE + 1)), width - 1);

    board.toggle_cell(row, col);

    draw_cells();
});



/// Rendering loop.
let frame_id = null;
const render_loop = () => {
    board.tick();

    draw_cells();

    frame_id = requestAnimationFrame(render_loop);
};

/// Draw the grid array.
const draw_grid = () => {
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
const draw_cells = () => {
    const cellsPtr = board.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, (width * height) / 8);

    ctx.beginPath();

    for (let row = 0; row < height; ++row) {
        for (let col = 0; col < width; ++col) {
            const idx = get_index(row, col);

            ctx.fillStyle = !bitIsSet(idx, cells) ?
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
const get_index = (row, column) => {
    return row * width + column;
};

/// Determine if the nth element of arr is set as true (alive).
const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
};



draw_grid();
draw_cells();
requestAnimationFrame(render_loop);
