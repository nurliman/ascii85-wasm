import { defineConfig } from "rollup";
import { babel } from "@rollup/plugin-babel";
import { wasm } from "@rollup/plugin-wasm";
import { terser } from "rollup-plugin-terser";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";

const isWatch = !!process.env.ROLLUP_WATCH;

const commonPlugins = [
  isWatch &&
    serve({
      contentBase: ["public", "dist"],
      open: true,
    }),
  isWatch && livereload("dist"),
  // !isWatch && terser(),
];

export default defineConfig([
  {
    input: "./index.js",
    output: [
      {
        file: "dist/index.js",
        format: "cjs",
        sourcemap: true,
      },
    ],
    treeshake: true,
    plugins: [
      wasm({
        fileName: "ascii85.wasm",
      }),
      ...commonPlugins,
    ],
  },
  {
    input: "./index.js",
    output: [
      {
        file: "dist/node.cjs",
        format: "cjs",
        sourcemap: true,
      },
    ],
    treeshake: true,
    plugins: [
      wasm({
        targetEnv: "node",
        fileName: "ascii85.wasm",
      }),
      ...commonPlugins,
    ],
  },
  {
    input: "./node.mjs",
    output: [
      {
        file: "dist/node.mjs",
        format: "esm",
        sourcemap: true,
        inlineDynamicImports: true,
      },
    ],
    treeshake: true,
    plugins: [
      wasm({
        targetEnv: "node",
        fileName: "ascii85.wasm",
      }),
      ...commonPlugins,
    ],
  },
]);
