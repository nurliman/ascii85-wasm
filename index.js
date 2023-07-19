import * as ascii85_wasm from "./pkg/ascii85_wasm.js";
import loadWasmAsync from "./pkg/ascii85_wasm_bg.wasm";

/** @type {Promise<ascii85_wasm> | null} */
let wasmBindgenPromise = null;

/**
 * Load the wasm bindgen asynchronously.
 *
 * @returns {Promise<ascii85_wasm.InitOutput>} - A Promise resolving to the wasm bindgen.
 */
const loadWasmBindgen = () => {
  if (wasmBindgenPromise !== null) return wasmBindgenPromise;
  return (wasmBindgenPromise = loadWasmAsync().then(ascii85_wasm.initSync));
};

/**
 * Encodes the input string to ASCII85 using the wasm bindgen.
 * @preserve
 * @param {string} input - The input string to encode.
 * @returns {Promise<string>} - A Promise resolving to the encoded ASCII85 string.
 */
export const encode = async (input) => {
  await loadWasmBindgen();
  return ascii85_wasm.encode(input);
};
