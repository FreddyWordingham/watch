/// Wasm requires async import.
import("./main.js")
    .catch(e => console.error("Error importing 'main.js':", e));
