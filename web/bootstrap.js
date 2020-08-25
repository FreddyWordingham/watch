/// Wasm requires async import.
import("./index.js")
    .catch(e => console.error("Error importing `index.js`:", e));
